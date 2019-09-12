use std::{fs::File, io::BufReader};

use rlua::{UserData, UserDataMethods};

use super::LuaTree;

pub struct Remodel;

impl UserData for Remodel {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("load", |_context, arg: String| {
            let file = BufReader::new(File::open(&arg).map_err(rlua::Error::external)?);
            let tree = rbx_xml::from_reader_default(file).map_err(rlua::Error::external)?;

            Ok(LuaTree::new(tree))
        });
    }
}
