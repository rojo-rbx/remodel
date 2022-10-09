use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, BufWriter, Read},
    path::Path,
    sync::Arc,
    time::Duration,
};

use mlua::{Lua, UserData, UserDataMethods};
use rbx_dom_weak::{types::VariantType, InstanceBuilder, WeakDom};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE, COOKIE, USER_AGENT},
    StatusCode,
};

use crate::{
    remodel_context::RemodelContext,
    roblox_api::LuaInstance,
    sniff_type::{sniff_type, DocumentType},
    value::{lua_to_rbxvalue, rbxvalue_to_lua, type_from_str},
};

fn xml_encode_options() -> rbx_xml::EncodeOptions {
    rbx_xml::EncodeOptions::new().property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown)
}

fn xml_decode_options() -> rbx_xml::DecodeOptions {
    rbx_xml::DecodeOptions::new().property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown)
}

pub enum UploadPlaceAsset {
    Legacy(u64),
    CloudAPI { place_id: u64, universe_id: u64 },
}

pub struct Remodel;

impl Remodel {
    fn read_xml_place_file<'lua>(context: &'lua Lua, path: &Path) -> mlua::Result<LuaInstance> {
        let file = BufReader::new(File::open(path).map_err(mlua::Error::external)?);
        let source_tree =
            rbx_xml::from_reader(file, xml_decode_options()).map_err(mlua::Error::external)?;

        Remodel::import_tree_root(context, source_tree)
    }

    fn read_xml_model_file<'lua>(
        context: &'lua Lua,
        path: &Path,
    ) -> mlua::Result<Vec<LuaInstance>> {
        let file = BufReader::new(File::open(path).map_err(mlua::Error::external)?);
        let source_tree =
            rbx_xml::from_reader(file, xml_decode_options()).map_err(mlua::Error::external)?;

        Remodel::import_tree_children(context, source_tree)
    }

    fn read_binary_place_file<'lua>(context: &'lua Lua, path: &Path) -> mlua::Result<LuaInstance> {
        let file = BufReader::new(File::open(path).map_err(mlua::Error::external)?);
        let source_tree = rbx_binary::from_reader(file).map_err(mlua::Error::external)?;

        Remodel::import_tree_root(context, source_tree)
    }

    fn read_binary_model_file<'lua>(
        context: &'lua Lua,
        path: &Path,
    ) -> mlua::Result<Vec<LuaInstance>> {
        let file = BufReader::new(File::open(path).map_err(mlua::Error::external)?);

        let source_tree = rbx_binary::from_reader(file)
            .map_err(|err| mlua::Error::external(format!("{:?}", err)))?;

        Remodel::import_tree_children(context, source_tree)
    }

    pub fn import_tree_children(
        context: &Lua,
        mut source_tree: WeakDom,
    ) -> mlua::Result<Vec<LuaInstance>> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

        let source_root_ref = source_tree.root_ref();
        let source_root = source_tree.get_by_ref(source_root_ref).unwrap();
        let source_children = source_root.children().to_vec();

        let master_root_ref = master_handle.root_ref();

        let instances = source_children
            .into_iter()
            .map(|id| {
                source_tree.transfer(id, &mut master_handle, master_root_ref);
                LuaInstance::new(Arc::clone(&master_tree), id)
            })
            .collect::<Vec<_>>();

        Ok(instances)
    }

    pub fn import_tree_root(context: &Lua, mut source_tree: WeakDom) -> mlua::Result<LuaInstance> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

        let source_children = source_tree.root().children().to_vec();
        let source_builder = InstanceBuilder::new("DataModel");

        let master_root_ref = master_handle.root_ref();
        let new_root_ref = master_handle.insert(master_root_ref, source_builder);

        for child_id in source_children {
            source_tree.transfer(child_id, &mut master_handle, new_root_ref);
        }

        Ok(LuaInstance::new(Arc::clone(&master_tree), new_root_ref))
    }

    fn write_xml_place_file(lua_instance: LuaInstance, path: &Path) -> mlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(mlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class != "DataModel" {
            return Err(mlua::Error::external(
                "Only DataModel instances can be saved as place files.",
            ));
        }

        rbx_xml::to_writer(file, &tree, instance.children(), xml_encode_options())
            .map_err(mlua::Error::external)?;

        Ok(())
    }

    fn write_binary_place_file(lua_instance: LuaInstance, path: &Path) -> mlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(mlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class != "DataModel" {
            return Err(mlua::Error::external(
                "Only DataModel instances can be saved as place files.",
            ));
        }

        rbx_binary::to_writer(file, &tree, instance.children()).map_err(mlua::Error::external)?;

        Ok(())
    }

    fn write_xml_model_file(lua_instance: LuaInstance, path: &Path) -> mlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(mlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(mlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_xml::to_writer(file, &tree, &[lua_instance.id], xml_encode_options())
            .map_err(mlua::Error::external)
    }

    fn write_binary_model_file(lua_instance: LuaInstance, path: &Path) -> mlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(mlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(mlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_binary::to_writer(file, &tree, &[lua_instance.id])
            .map_err(|err| mlua::Error::external(format!("{:?}", err)))
    }

    fn read_model_asset(context: &Lua, asset_id: u64) -> mlua::Result<Vec<LuaInstance>> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie();
        let url = format!("https://assetdelivery.roblox.com/v1/asset/?id={}", asset_id);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 3))
            .build()
            .map_err(mlua::Error::external)?;

        let mut request = client.get(&url);

        if let Some(auth_cookie) = auth_cookie {
            request = request.header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie));
        } else {
            log::warn!("No auth cookie detected, Remodel may be unable to download this asset.");
        }

        let mut response = request.send().map_err(mlua::Error::external)?;

        let mut body = Vec::new();
        response
            .read_to_end(&mut body)
            .map_err(mlua::Error::external)?;

        let source_tree = match sniff_type(&body) {
            Some(DocumentType::Binary) => {
                rbx_binary::from_reader(body.as_slice()).map_err(mlua::Error::external)?
            }

            Some(DocumentType::Xml) => rbx_xml::from_reader(body.as_slice(), xml_decode_options())
                .map_err(mlua::Error::external)?,

            None => {
                let first_few_bytes: Vec<_> = body.iter().copied().take(20).collect();
                let snippet = std::str::from_utf8(first_few_bytes.as_slice());

                let message = format!(
                    "Unknown response trying to read model asset ID {}. First few bytes:\n{:?}",
                    asset_id, snippet
                );

                return Err(mlua::Error::external(message));
            }
        };

        Remodel::import_tree_children(context, source_tree)
    }

    fn read_place_asset(context: &Lua, asset_id: u64) -> mlua::Result<LuaInstance> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie();
        let url = format!("https://assetdelivery.roblox.com/v1/asset/?id={}", asset_id);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 3))
            .build()
            .map_err(mlua::Error::external)?;
        let mut request = client.get(&url);

        if let Some(auth_cookie) = auth_cookie {
            request = request.header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie));
        } else {
            log::warn!("No auth cookie detected, Remodel may be unable to download this asset.");
        }

        let mut response = request.send().map_err(mlua::Error::external)?;

        let mut body = Vec::new();
        response
            .read_to_end(&mut body)
            .map_err(mlua::Error::external)?;

        let source_tree = match sniff_type(&body) {
            Some(DocumentType::Binary) => {
                rbx_binary::from_reader(body.as_slice()).map_err(mlua::Error::external)?
            }

            Some(DocumentType::Xml) => rbx_xml::from_reader(body.as_slice(), xml_decode_options())
                .map_err(mlua::Error::external)?,

            None => {
                let snippet = std::str::from_utf8(body.as_slice());

                let message = format!(
                    "Unknown response trying to read model asset ID {}. Response is:\n{:?}",
                    asset_id, snippet
                );

                return Err(mlua::Error::external(message));
            }
        };

        Remodel::import_tree_root(context, source_tree)
    }

    fn write_existing_model_asset(
        context: &Lua,
        lua_instance: LuaInstance,
        asset_id: u64,
    ) -> mlua::Result<()> {
        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(mlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        let mut buffer = Vec::new();
        rbx_binary::to_writer(&mut buffer, &tree, &[lua_instance.id])
            .map_err(mlua::Error::external)?;

        Remodel::upload_asset(context, buffer, asset_id)
    }

    fn write_existing_place_asset(
        context: &Lua,
        lua_instance: LuaInstance,
        asset: UploadPlaceAsset,
    ) -> mlua::Result<()> {
        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| mlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class != "DataModel" {
            return Err(mlua::Error::external(
                "Only DataModel instances can be saved as place files.",
            ));
        }

        let mut buffer = Vec::new();
        rbx_binary::to_writer(&mut buffer, &tree, instance.children())
            .map_err(mlua::Error::external)?;

        match asset {
            UploadPlaceAsset::Legacy(asset_id) => Remodel::upload_asset(context, buffer, asset_id),
            UploadPlaceAsset::CloudAPI {
                place_id,
                universe_id,
            } => Remodel::cloud_upload_place_asset(context, buffer, universe_id, place_id),
        }
    }

    fn cloud_upload_place_asset(
        context: &Lua,
        buffer: Vec<u8>,
        universe_id: u64,
        asset_id: u64,
    ) -> mlua::Result<()> {
        let re_context = RemodelContext::get(context)?;
        let url = format!(
            "https://apis.roblox.com/universes/v1/{}/places/{}/versions?versionType=Published",
            universe_id, asset_id
        );

        let api_key = re_context.api_key().ok_or_else(|| {
            mlua::Error::external(
                "Uploading a place asset using the Cloud API requires an auth key be set via --api-key or REMODEL_AUTH_KEY environment variable",
            )
        })?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 3))
            .build()
            .map_err(mlua::Error::external)?;

        let build_request = move || {
            client
                .post(&url)
                .header("x-api-key", api_key)
                .header(CONTENT_TYPE, "application/xml")
                .header(ACCEPT, "application/json")
                .body(buffer.clone())
        };

        log::debug!("Uploading to Roblox Cloud...");
        let response = build_request().send().map_err(mlua::Error::external)?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(mlua::Error::external(format!(
                "Roblox API returned an error, status {}.",
                response.status()
            )))
        }
    }

    fn upload_asset(context: &Lua, buffer: Vec<u8>, asset_id: u64) -> mlua::Result<()> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie().ok_or_else(|| {
            mlua::Error::external(
                "Uploading assets requires an auth cookie, please log into Roblox Studio.",
            )
        })?;

        let url = format!(
            "https://data.roblox.com/Data/Upload.ashx?assetid={}",
            asset_id
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60 * 3))
            .build()
            .map_err(mlua::Error::external)?;

        let build_request = move || {
            client
                .post(&url)
                .header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
                .header(USER_AGENT, "Roblox/WinInet")
                .header(CONTENT_TYPE, "application/xml")
                .header(ACCEPT, "application/json")
                .body(buffer.clone())
        };

        log::debug!("Uploading to Roblox...");
        let mut response = build_request().send().map_err(mlua::Error::external)?;

        // Starting in Feburary, 2021, the upload endpoint performs CSRF challenges.
        // If we receive an HTTP 403 with a X-CSRF-Token reply, we should retry the
        // request, echoing the value of that header.
        if response.status() == StatusCode::FORBIDDEN {
            if let Some(csrf_token) = response.headers().get("X-CSRF-Token") {
                log::debug!("Received CSRF challenge, retrying with token...");
                response = build_request()
                    .header("X-CSRF-Token", csrf_token)
                    .send()
                    .map_err(mlua::Error::external)?;
            }
        }

        if response.status().is_success() {
            Ok(())
        } else {
            Err(mlua::Error::external(format!(
                "Roblox API returned an error, status {}.",
                response.status()
            )))
        }
    }

    fn get_raw_property<'a>(
        context: &'a Lua,
        lua_instance: LuaInstance,
        name: &str,
    ) -> mlua::Result<mlua::Value<'a>> {
        let tree = lua_instance.tree.lock().unwrap();

        let instance = tree.get_by_ref(lua_instance.id).ok_or_else(|| {
            mlua::Error::external("Cannot call remodel.getRawProperty on a destroyed instance.")
        })?;

        match instance.properties.get(name) {
            Some(value) => rbxvalue_to_lua(context, value),
            None => Ok(mlua::Value::Nil),
        }
    }

    fn set_raw_property(
        lua_instance: LuaInstance,
        key: String,
        ty: VariantType,
        lua_value: mlua::Value<'_>,
    ) -> mlua::Result<()> {
        let mut tree = lua_instance.tree.lock().unwrap();

        let instance = tree.get_by_ref_mut(lua_instance.id).ok_or_else(|| {
            mlua::Error::external("Cannot call remodel.setRawProperty on a destroyed instance.")
        })?;

        let value = lua_to_rbxvalue(ty, lua_value)?;
        instance.properties.insert(key, value);

        Ok(())
    }
}

impl UserData for Remodel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "getRawProperty",
            |context, (instance, name): (LuaInstance, String)| {
                Self::get_raw_property(context, instance, &name)
            },
        );

        methods.add_function(
            "setRawProperty",
            |_context, (instance, name, lua_ty, value): (LuaInstance, String, String, mlua::Value<'_>)| {
                let ty = type_from_str(&lua_ty)
                    .ok_or_else(|| mlua::Error::external(format!("{} is not a valid Roblox type.", lua_ty)))?;

                Self::set_raw_property(instance, name, ty, value)
            },
        );

        methods.add_function("readPlaceFile", |context, lua_path: String| {
            let path = Path::new(&lua_path);

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxlx") => Remodel::read_xml_place_file(context, path),
                Some("rbxl") => Remodel::read_binary_place_file(context, path),
                _ => Err(mlua::Error::external(format!(
                    "Invalid place file path {}",
                    path.display()
                ))),
            }
        });

        methods.add_function("readModelFile", |context, lua_path: String| {
            let path = Path::new(&lua_path);

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxmx") => Remodel::read_xml_model_file(context, path),
                Some("rbxm") => Remodel::read_binary_model_file(context, path),
                _ => Err(mlua::Error::external(format!(
                    "Invalid model file path {}",
                    path.display()
                ))),
            }
        });

        methods.add_function("readModelAsset", |context, asset_id: String| {
            let asset_id = asset_id.parse().map_err(mlua::Error::external)?;

            Remodel::read_model_asset(context, asset_id)
        });

        methods.add_function("readPlaceAsset", |context, asset_id: String| {
            let asset_id = asset_id.parse().map_err(mlua::Error::external)?;

            Remodel::read_place_asset(context, asset_id)
        });

        methods.add_function(
            "writeExistingModelAsset",
            |context, (instance, asset_id): (LuaInstance, String)| {
                let asset_id = asset_id.parse().map_err(mlua::Error::external)?;

                Remodel::write_existing_model_asset(context, instance, asset_id)
            },
        );

        methods.add_function(
            "writeExistingPlaceAsset",
            |context, (instance, asset_id): (LuaInstance, String)| {
                let asset_id = asset_id.parse().map_err(mlua::Error::external)?;

                Remodel::write_existing_place_asset(
                    context,
                    instance,
                    UploadPlaceAsset::Legacy(asset_id),
                )
            },
        );

        methods.add_function(
            "publishPlaceToUniverse",
            |context, (instance, universe_id, place_id): (LuaInstance, u64, u64)| {
                Remodel::write_existing_place_asset(
                    context,
                    instance,
                    UploadPlaceAsset::CloudAPI {
                        universe_id,
                        place_id,
                    },
                )
            },
        );

        methods.add_function(
            "writePlaceFile",
            |_context, (lua_path, instance): (String, LuaInstance)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxlx") => Remodel::write_xml_place_file(instance, path),
                    Some("rbxl") => Remodel::write_binary_place_file(instance, path),
                    _ => Err(mlua::Error::external(format!(
                        "Invalid place file path {}",
                        path.display()
                    ))),
                }
            },
        );

        methods.add_function(
            "writeModelFile",
            |_context, (lua_path, instance): (String, LuaInstance)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxmx") => Remodel::write_xml_model_file(instance, path),
                    Some("rbxm") => Remodel::write_binary_model_file(instance, path),
                    _ => Err(mlua::Error::external(format!(
                        "Invalid model file path {}",
                        path.display()
                    ))),
                }
            },
        );

        methods.add_function("readFile", |_context, path: String| {
            fs::read_to_string(path).map_err(mlua::Error::external)
        });

        methods.add_function("readDir", |_context, path: String| {
            fs::read_dir(path)
                .map_err(mlua::Error::external)?
                .filter_map(|entry| {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => return Some(Err(mlua::Error::external(err))),
                    };

                    match entry.file_name().into_string() {
                        Ok(name) => Some(Ok(name)),
                        Err(bad_name) => {
                            log::warn!(
                                "Encountered invalid Unicode file name {:?}, skipping.",
                                bad_name
                            );
                            None
                        }
                    }
                })
                .collect::<Result<Vec<String>, _>>()
        });

        methods.add_function(
            "writeFile",
            |_context, (path, contents): (String, mlua::String)| {
                fs::write(path, contents.as_bytes()).map_err(mlua::Error::external)
            },
        );

        methods.add_function("createDirAll", |_context, path: String| {
            fs::create_dir_all(path).map_err(mlua::Error::external)
        });

        methods.add_function("isFile", |_context, path: String| {
            let meta = fs::metadata(path).map_err(mlua::Error::external)?;
            Ok(meta.is_file())
        });

        methods.add_function("isDir", |_context, path: String| {
            let meta = fs::metadata(path).map_err(mlua::Error::external)?;
            Ok(meta.is_dir())
        });

        methods.add_function("removeFile", |_context, path: String| {
            fs::remove_file(path).map_err(mlua::Error::external)
        });

        methods.add_function("removeDir", |_context, path: String| {
            fs::remove_dir_all(path).map_err(mlua::Error::external)
        });
    }
}
