use std::sync::{Arc, Mutex};

use rbx_dom_weak::RbxTree;
use rlua::{UserData, UserDataMethods};

use super::LuaInstance;

pub struct LuaTree(Arc<Mutex<RbxTree>>);

impl LuaTree {
    pub fn new(tree: RbxTree) -> Self {
        LuaTree(Arc::new(Mutex::new(tree)))
    }
}

impl UserData for LuaTree {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getRootInstance", |_context, this, _args: ()| {
            let root_id = this.0.lock().unwrap().get_root_id();

            Ok(LuaInstance::new(Arc::clone(&this.0), root_id))
        });
    }
}
