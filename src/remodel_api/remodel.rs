use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, BufWriter},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use rbx_dom_weak::{types::VariantType, InstanceBuilder, WeakDom};
use reqwest::header::{CONTENT_TYPE, COOKIE, USER_AGENT};
use rlua::{Context, UserData, UserDataMethods};

use crate::{
    remodel_context::RemodelContext,
    roblox_api::LuaInstance,
    value::{lua_to_rbxvalue, rbxvalue_to_lua, type_from_str},
};

fn xml_encode_options() -> rbx_xml::EncodeOptions {
    rbx_xml::EncodeOptions::new().property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown)
}

fn xml_decode_options() -> rbx_xml::DecodeOptions {
    rbx_xml::DecodeOptions::new().property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown)
}

pub struct Remodel;

impl Remodel {
    fn read_xml_place_file<'lua>(context: Context<'lua>, path: &Path) -> rlua::Result<LuaInstance> {
        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);
        let source_tree =
            rbx_xml::from_reader(file, xml_decode_options()).map_err(rlua::Error::external)?;

        Remodel::import_tree_root(context, source_tree)
    }

    fn read_xml_model_file<'lua>(
        context: Context<'lua>,
        path: &Path,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);
        let source_tree =
            rbx_xml::from_reader(file, xml_decode_options()).map_err(rlua::Error::external)?;

        Remodel::import_tree_children(context, source_tree)
    }

    fn read_binary_model_file<'lua>(
        context: Context<'lua>,
        path: &Path,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);

        let source_tree = rbx_binary::from_reader_default(file)
            .map_err(|err| rlua::Error::external(format!("{:?}", err)))?;

        Remodel::import_tree_children(context, source_tree)
    }

    pub fn import_tree_children(
        context: Context<'_>,
        mut source_tree: WeakDom,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

        let source_root_ref = source_tree.root_ref();
        let source_root = source_tree.get_by_ref(source_root_ref).unwrap();
        let source_children = source_root.children().to_vec();

        let master_root_ref = master_handle.root_ref();

        let instances = source_children
            .into_iter()
            .map(|id| {
                // FIXME
                // source_tree.move_instance(id, &mut master_handle, master_root_ref);
                LuaInstance::new(Arc::clone(&master_tree), id)
            })
            .collect::<Vec<_>>();

        Ok(instances)
    }

    pub fn import_tree_root(
        context: Context<'_>,
        mut source_tree: WeakDom,
    ) -> rlua::Result<LuaInstance> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

        let source_root_ref = source_tree.root_ref();
        let source_root = source_tree.get_by_ref(source_root_ref).unwrap();
        let source_children = source_root.children().to_vec();
        let source_builder: InstanceBuilder = unimplemented!();

        let master_root_ref = master_handle.root_ref();
        let new_root_ref = master_handle.insert(master_root_ref, source_builder);

        for child_id in source_children {
            // FIXME
            // source_tree.move_instance(child_id, &mut master_handle, new_root_ref);
        }

        Ok(LuaInstance::new(Arc::clone(&master_tree), new_root_ref))
    }

    fn write_xml_place_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class != "DataModel" {
            return Err(rlua::Error::external(
                "Only DataModel instances can be saved as place files.",
            ));
        }

        rbx_xml::to_writer(file, &tree, instance.children(), xml_encode_options())
            .map_err(rlua::Error::external)?;

        Ok(())
    }

    fn write_xml_model_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(rlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_xml::to_writer(file, &tree, &[lua_instance.id], xml_encode_options())
            .map_err(rlua::Error::external)
    }

    fn write_binary_model_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(rlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_binary::to_writer_default(file, &tree, &[lua_instance.id])
            .map_err(|err| rlua::Error::external(format!("{:?}", err)))
    }

    fn read_model_asset(context: Context<'_>, asset_id: u64) -> rlua::Result<Vec<LuaInstance>> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie();
        let url = format!("https://www.roblox.com/asset/?id={}", asset_id);

        let client = reqwest::Client::new();
        let mut request = client.get(&url);

        if let Some(auth_cookie) = auth_cookie {
            request = request.header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie));
        } else {
            log::warn!("No auth cookie detected, Remodel may be unable to download this asset.");
        }

        let response = request.send().map_err(rlua::Error::external)?;

        let source_tree =
            rbx_xml::from_reader(response, xml_decode_options()).map_err(rlua::Error::external)?;

        Remodel::import_tree_children(context, source_tree)
    }

    fn read_place_asset(context: Context<'_>, asset_id: u64) -> rlua::Result<LuaInstance> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie();
        let url = format!("https://www.roblox.com/asset/?id={}", asset_id);

        let client = reqwest::Client::new();
        let mut request = client.get(&url);

        if let Some(auth_cookie) = auth_cookie {
            request = request.header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie));
        } else {
            log::warn!("No auth cookie detected, Remodel may be unable to download this asset.");
        }

        let response = request.send().map_err(rlua::Error::external)?;

        let source_tree =
            rbx_xml::from_reader(response, xml_decode_options()).map_err(rlua::Error::external)?;

        Remodel::import_tree_root(context, source_tree)
    }

    fn write_existing_model_asset(
        context: Context<'_>,
        lua_instance: LuaInstance,
        asset_id: u64,
    ) -> rlua::Result<()> {
        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class == "DataModel" {
            return Err(rlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        let mut buffer = Vec::new();
        rbx_xml::to_writer(&mut buffer, &tree, &[lua_instance.id], xml_encode_options())
            .map_err(rlua::Error::external)?;

        Remodel::upload_asset(context, buffer, asset_id)
    }

    fn write_existing_place_asset(
        context: Context<'_>,
        lua_instance: LuaInstance,
        asset_id: u64,
    ) -> rlua::Result<()> {
        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_by_ref(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Cannot save a destroyed instance."))?;

        if instance.class != "DataModel" {
            return Err(rlua::Error::external(
                "Only DataModel instances can be saved as place files.",
            ));
        }

        let mut buffer = Vec::new();
        rbx_xml::to_writer(
            &mut buffer,
            &tree,
            instance.children(),
            xml_encode_options(),
        )
        .map_err(rlua::Error::external)?;

        Remodel::upload_asset(context, buffer, asset_id)
    }

    fn upload_asset(context: Context<'_>, buffer: Vec<u8>, asset_id: u64) -> rlua::Result<()> {
        let re_context = RemodelContext::get(context)?;
        let auth_cookie = re_context.auth_cookie().ok_or_else(|| {
            rlua::Error::external(
                "Uploading assets requires an auth cookie, please log into Roblox Studio.",
            )
        })?;

        let url = format!(
            "https://data.roblox.com/Data/Upload.ashx?assetid={}",
            asset_id
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header(COOKIE, format!(".ROBLOSECURITY={}", auth_cookie))
            .header(CONTENT_TYPE, "application/xml")
            .header(USER_AGENT, "Roblox/WinInet")
            .body(buffer)
            .send()
            .map_err(rlua::Error::external)?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(rlua::Error::external(format!(
                "Roblox API returned an error, status {}.",
                response.status()
            )))
        }
    }

    fn get_raw_property<'a>(
        context: Context<'a>,
        lua_instance: LuaInstance,
        name: &str,
    ) -> rlua::Result<rlua::Value<'a>> {
        let tree = lua_instance.tree.lock().unwrap();

        let instance = tree.get_by_ref(lua_instance.id).ok_or_else(|| {
            rlua::Error::external("Cannot call remodel.getRawProperty on a destroyed instance.")
        })?;

        match instance.properties.get(name) {
            Some(value) => rbxvalue_to_lua(context, value),
            None => Ok(rlua::Value::Nil),
        }
    }

    fn set_raw_property(
        lua_instance: LuaInstance,
        key: String,
        ty: VariantType,
        lua_value: rlua::Value<'_>,
    ) -> rlua::Result<()> {
        let mut tree = lua_instance.tree.lock().unwrap();

        let instance = tree.get_by_ref_mut(lua_instance.id).ok_or_else(|| {
            rlua::Error::external("Cannot call remodel.setRawProperty on a destroyed instance.")
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
            |_context, (instance, name, lua_ty, value): (LuaInstance, String, String, rlua::Value<'_>)| {
                let ty = type_from_str(&lua_ty)
                    .ok_or_else(|| rlua::Error::external(format!("{} is not a valid Roblox type.", lua_ty)))?;

                Self::set_raw_property(instance, name, ty, value)
            },
        );

        methods.add_function("readPlaceFile", |context, lua_path: String| {
            let path = Path::new(&lua_path);

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxlx") => Remodel::read_xml_place_file(context, path),
                Some("rbxl") => Err(rlua::Error::external(
                    "Reading rbxl place files is not supported yet.",
                )),
                _ => Err(rlua::Error::external(format!(
                    "Invalid place file path {}",
                    path.display()
                ))),
            }
        });

        methods.add_function("readModelFile", |context, lua_path: String| {
            let path = Path::new(&lua_path);

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxmx") => Remodel::read_xml_model_file(context, path),
                Some("rbxm") => {
                    log::warn!(
                        "rbxm model support in Remodel is limited. rbxmx models are recommended."
                    );

                    Remodel::read_binary_model_file(context, path)
                }
                _ => Err(rlua::Error::external(format!(
                    "Invalid model file path {}",
                    path.display()
                ))),
            }
        });

        methods.add_function("readModelAsset", |context, asset_id: String| {
            let asset_id = asset_id.parse().map_err(rlua::Error::external)?;

            Remodel::read_model_asset(context, asset_id)
        });

        methods.add_function("readPlaceAsset", |context, asset_id: String| {
            let asset_id = asset_id.parse().map_err(rlua::Error::external)?;

            Remodel::read_place_asset(context, asset_id)
        });

        methods.add_function(
            "writeExistingModelAsset",
            |context, (instance, asset_id): (LuaInstance, String)| {
                let asset_id = asset_id.parse().map_err(rlua::Error::external)?;

                Remodel::write_existing_model_asset(context, instance, asset_id)
            },
        );

        methods.add_function(
            "writeExistingPlaceAsset",
            |context, (instance, asset_id): (LuaInstance, String)| {
                let asset_id = asset_id.parse().map_err(rlua::Error::external)?;

                Remodel::write_existing_place_asset(context, instance, asset_id)
            },
        );

        methods.add_function(
            "writePlaceFile",
            |_context, (instance, lua_path): (LuaInstance, String)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxlx") => Remodel::write_xml_place_file(instance, path),
                    Some("rbxl") => Err(rlua::Error::external(
                        "Writing rbxl place files is not supported yet.",
                    )),
                    _ => Err(rlua::Error::external(format!(
                        "Invalid place file path {}",
                        path.display()
                    ))),
                }
            },
        );

        methods.add_function(
            "writeModelFile",
            |_context, (instance, lua_path): (LuaInstance, String)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxmx") => Remodel::write_xml_model_file(instance, path),
                    Some("rbxm") => {
                        log::warn!("rbxm model support in Remodel is limited. rbxmx models are recommended.");

                        Remodel::write_binary_model_file(instance, path)
                    },
                    _ => Err(rlua::Error::external(format!(
                        "Invalid model file path {}",
                        path.display()
                    ))),
                }
            },
        );

        methods.add_function("readFile", |_context, path: String| {
            fs::read_to_string(path).map_err(rlua::Error::external)
        });

        methods.add_function("readDir", |_context, path: String| {
            fs::read_dir(path)
                .map_err(rlua::Error::external)?
                .filter_map(|entry| {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => return Some(Err(rlua::Error::external(err))),
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
            |_context, (path, contents): (String, rlua::String)| {
                fs::write(path, contents.as_bytes()).map_err(rlua::Error::external)
            },
        );

        methods.add_function("createDirAll", |_context, path: String| {
            fs::create_dir_all(path).map_err(rlua::Error::external)
        });

        methods.add_function("isFile", |_context, path: String| {
            let meta = fs::metadata(path).map_err(rlua::Error::external)?;
            Ok(meta.is_file())
        });

        methods.add_function("isDir", |_context, path: String| {
            let meta = fs::metadata(path).map_err(rlua::Error::external)?;
            Ok(meta.is_dir())
        });
    }
}
