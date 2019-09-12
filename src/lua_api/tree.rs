use std::sync::Arc;

use rbx_dom_weak::RbxTree;
use rlua::{UserData, UserDataMethods};

use super::LuaInstance;

pub struct LuaTree(Arc<RbxTree>);

impl LuaTree {
    pub fn new(tree: RbxTree) -> Self {
        LuaTree(Arc::new(tree))
    }
}

impl UserData for LuaTree {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("getRootInstance", |_context, this, _args: ()| {
            Ok(LuaInstance::new(Arc::clone(&this.0), this.0.get_root_id()))
        });
    }
}
