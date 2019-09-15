use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, BufWriter},
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree};
use rlua::{UserData, UserDataMethods};

use super::LuaInstance;

pub struct Remodel;

impl UserData for Remodel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        let master_tree_original = Arc::new(Mutex::new(RbxTree::new(RbxInstanceProperties {
            name: "REMODEL ROOT".to_owned(),
            class_name: "REMODEL ROOT".to_owned(),
            properties: HashMap::new(),
        })));

        let master_tree = master_tree_original.clone();
        methods.add_function("readPlaceFile", move |_context, lua_path: String| {
            let path = Path::new(&lua_path);

            let mut master_handle = master_tree.lock().unwrap();

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxlx") => {
                    let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);

                    let mut source_tree =
                        rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

                    let source_root_id = source_tree.get_root_id();
                    let source_root = source_tree.get_instance(source_root_id).unwrap();
                    let source_children = source_root.get_children_ids().to_vec();

                    let master_root_id = master_handle.get_root_id();
                    let new_root_id =
                        master_handle.insert_instance(source_root.deref().clone(), master_root_id);

                    for child_id in source_children {
                        source_tree.move_instance(child_id, &mut master_handle, new_root_id);
                    }

                    Ok(LuaInstance::new(Arc::clone(&master_tree), new_root_id))
                }
                Some("rbxl") => Err(rlua::Error::external(
                    "Reading rbxl place files is not supported yet.",
                )),
                _ => Err(rlua::Error::external(format!(
                    "Invalid place file path {}",
                    lua_path
                ))),
            }
        });

        let master_tree = master_tree_original.clone();
        methods.add_function("readModelFile", move |_context, lua_path: String| {
            let path = Path::new(&lua_path);

            let mut master_handle = master_tree.lock().unwrap();

            match path.extension().and_then(OsStr::to_str) {
                Some("rbxmx") => {
                    let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);
                    let mut source_tree =
                        rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

                    let source_root_id = source_tree.get_root_id();
                    let source_root = source_tree.get_instance(source_root_id).unwrap();
                    let source_children = source_root.get_children_ids().to_vec();

                    let master_root_id = master_handle.get_root_id();

                    let instances = source_children
                        .into_iter()
                        .map(|id| {
                            source_tree.move_instance(id, &mut master_handle, master_root_id);
                            LuaInstance::new(Arc::clone(&master_tree), id)
                        })
                        .collect::<Vec<_>>();

                    Ok(instances)
                }
                Some("rbxm") => Err(rlua::Error::external(
                    "Reading rbxm models files is not supported yet.",
                )),
                _ => Err(rlua::Error::external(format!(
                    "Invalid model file path {}",
                    lua_path
                ))),
            }
        });

        methods.add_function(
            "writePlaceFile",
            |_context, (lua_instance, lua_path): (LuaInstance, String)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxlx") => {
                        let file =
                            BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

                        let tree = lua_instance.tree.lock().unwrap();
                        let instance = tree
                            .get_instance(lua_instance.id)
                            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

                        if instance.class_name != "DataModel" {
                            return Err(rlua::Error::external(
                                "Only DataModel instances can be saved as place files.",
                            ));
                        }

                        rbx_xml::to_writer_default(file, &tree, instance.get_children_ids())
                            .map_err(rlua::Error::external)?;

                        Ok(())
                    }
                    Some("rbxl") => Err(rlua::Error::external(
                        "Writing rbxl place files is not supported yet.",
                    )),
                    _ => Err(rlua::Error::external(format!(
                        "Invalid place file path {}",
                        lua_path
                    ))),
                }
            },
        );

        methods.add_function(
            "writeModelFile",
            |_context, (lua_instance, lua_path): (LuaInstance, String)| {
                let path = Path::new(&lua_path);

                match path.extension().and_then(OsStr::to_str) {
                    Some("rbxmx") => {
                        let file =
                            BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

                        let tree = lua_instance.tree.lock().unwrap();
                        let instance = tree
                            .get_instance(lua_instance.id)
                            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

                        if instance.class_name == "DataModel" {
                            return Err(rlua::Error::external(
                                "DataModel instances must be saved as place files, not model files.",
                            ));
                        }

                        rbx_xml::to_writer_default(file, &tree, &[lua_instance.id])
                            .map_err(rlua::Error::external)
                    }
                    Some("rbxm") => Err(rlua::Error::external(
                        "Writing rbxm model files is not supported yet.",
                    )),
                    _ => Err(rlua::Error::external(format!(
                        "Invalid model file path {}",
                        lua_path
                    ))),
                }
            },
        );

        methods.add_function("readFile", |_context, path: String| {
            fs::read_to_string(path).map_err(rlua::Error::external)
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
    }
}
