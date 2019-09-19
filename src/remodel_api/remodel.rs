use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, BufWriter},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree};
use rlua::{Context, UserData, UserDataMethods};

use super::LuaInstance;
use crate::remodel_context::RemodelContext;

pub struct Remodel;

impl Remodel {
    fn read_xml_place_file<'lua>(context: Context<'lua>, path: &Path) -> rlua::Result<LuaInstance> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);

        let mut source_tree = rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

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

    fn read_xml_model_file<'lua>(
        context: Context<'lua>,
        path: &Path,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);
        let source_tree = rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

        Remodel::import_model_tree(context, source_tree)
    }

    fn read_binary_model_file<'lua>(
        context: Context<'lua>,
        path: &Path,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let mut source_tree = RbxTree::new(RbxInstanceProperties {
            name: "TEMP_RBX_BINARY_ROOT".to_owned(),
            class_name: "DataModel".to_owned(),
            properties: Default::default(),
        });
        let root_id = source_tree.get_root_id();

        let file = BufReader::new(File::open(path).map_err(rlua::Error::external)?);

        rbx_binary::decode(&mut source_tree, root_id, file)
            .map_err(|err| rlua::Error::external(format!("{:?}", err)))?;

        Remodel::import_model_tree(context, source_tree)
    }

    fn import_model_tree<'lua>(
        context: Context<'lua>,
        mut source_tree: RbxTree,
    ) -> rlua::Result<Vec<LuaInstance>> {
        let master_tree = RemodelContext::get(context)?.master_tree;
        let mut master_handle = master_tree.lock().unwrap();

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

    fn write_xml_place_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

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

    fn write_xml_model_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_instance(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        if instance.class_name == "DataModel" {
            return Err(rlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_xml::to_writer_default(file, &tree, &[lua_instance.id]).map_err(rlua::Error::external)
    }

    fn write_binary_model_file(lua_instance: LuaInstance, path: &Path) -> rlua::Result<()> {
        let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

        let tree = lua_instance.tree.lock().unwrap();
        let instance = tree
            .get_instance(lua_instance.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        if instance.class_name == "DataModel" {
            return Err(rlua::Error::external(
                "DataModel instances must be saved as place files, not model files.",
            ));
        }

        rbx_binary::encode(&tree, &[lua_instance.id], file)
            .map_err(|err| rlua::Error::external(format!("{:?}", err)))
    }
}

impl UserData for Remodel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("readPlaceFile", move |context, lua_path: String| {
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

        methods.add_function("readModelFile", move |context, lua_path: String| {
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
