use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    sync::{Arc, Mutex},
};

use rlua::{UserData, UserDataMethods};

use super::LuaInstance;

pub struct Remodel;

impl UserData for Remodel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("load", |_context, path: String| {
            let file = BufReader::new(File::open(&path).map_err(rlua::Error::external)?);
            let tree = rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

            let root_id = tree.get_root_id();
            let tree = Arc::new(Mutex::new(tree));

            Ok(LuaInstance::new(tree, root_id))
        });

        methods.add_function(
            "save",
            |_context, (lua_instance, path): (LuaInstance, String)| {
                let file = BufWriter::new(File::create(&path).map_err(rlua::Error::external)?);

                let tree = lua_instance.tree.lock().unwrap();
                let instance = tree
                    .get_instance(lua_instance.id)
                    .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

                let result = if instance.class_name == "DataModel" {
                    rbx_xml::to_writer_default(file, &tree, instance.get_children_ids())
                } else {
                    rbx_xml::to_writer_default(file, &tree, &[lua_instance.id])
                };

                result.map_err(rlua::Error::external)
            },
        );

        methods.add_function("createDirAll", |_context, path: String| {
            create_dir_all(path).map_err(rlua::Error::external)
        });
    }
}
