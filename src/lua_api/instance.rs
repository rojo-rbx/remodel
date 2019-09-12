use std::sync::Arc;

use rbx_dom_weak::{RbxId, RbxTree};
use rlua::{MetaMethod, ToLua, UserData, UserDataMethods};

pub struct LuaInstance {
    tree: Arc<RbxTree>,
    id: RbxId,
}

impl LuaInstance {
    pub fn new(tree: Arc<RbxTree>, id: RbxId) -> Self {
        LuaInstance { tree, id }
    }
}

impl UserData for LuaInstance {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("FindFirstChild", |_context, _this, _arg: String| {
            println!("Hello, world, from LuaInstance::hello!");

            Ok(())
        });

        methods.add_method("GetChildren", |_context, this, _args: ()| {
            let instance = this
                .tree
                .get_instance(this.id)
                .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

            let children: Vec<LuaInstance> = instance
                .get_children_ids()
                .into_iter()
                .map(|id| LuaInstance::new(Arc::clone(&this.tree), *id))
                .collect();

            Ok(children)
        });

        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            let instance = this
                .tree
                .get_instance(this.id)
                .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

            Ok(instance.name.as_str().to_lua(context))
        });

        methods.add_meta_method(MetaMethod::Index, |context, this, arg: String| {
            let instance = this
                .tree
                .get_instance(this.id)
                .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

            match arg.as_str() {
                "Name" => instance.name.as_str().to_lua(context),
                "ClassName" => instance.class_name.as_str().to_lua(context),
                _ => Err(rlua::Error::external(format!(
                    "'{}' is not a valid member of Instance",
                    arg
                ))),
            }
        });
    }
}
