use std::sync::Arc;

use rbx_dom_weak::RbxInstanceProperties;
use rlua::{Context, UserData, UserDataMethods};

use crate::{remodel_api::LuaInstance, remodel_context::RemodelContext};

pub struct RobloxApi;

impl RobloxApi {
    pub fn inject(context: Context<'_>) -> rlua::Result<()> {
        context.globals().set("Instance", Instance)?;

        Ok(())
    }
}

struct Instance;

impl UserData for Instance {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("new", |context, class_name: String| {
            let master_tree = RemodelContext::get(context)?.master_tree;
            let mut master_handle = master_tree.lock().unwrap();

            let properties = RbxInstanceProperties {
                name: class_name.clone(),
                class_name,
                properties: Default::default(),
            };

            let root_id = master_handle.get_root_id();
            let id = master_handle.insert_instance(properties, root_id);

            Ok(LuaInstance::new(Arc::clone(&master_tree), id))
        });
    }
}
