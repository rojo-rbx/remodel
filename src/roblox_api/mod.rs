mod instance;

use std::sync::Arc;

use rbx_dom_weak::InstanceBuilder;
use rlua::{Context, UserData, UserDataMethods};

use crate::remodel_context::RemodelContext;

pub use instance::LuaInstance;

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
            let database = rbx_reflection_database::get();

            if !database.classes.contains_key(class_name.as_str()) {
                return Err(rlua::Error::external(format!(
                    "'{}' is not a valid class of Instance.",
                    class_name,
                )));
            }

            let master_tree = RemodelContext::get(context)?.master_tree;
            let mut master_handle = master_tree.lock().unwrap();

            let builder = InstanceBuilder::new(class_name);

            let root_id = master_handle.root_ref();
            let id = master_handle.insert(root_id, builder);

            Ok(LuaInstance::new(Arc::clone(&master_tree), id))
        });
    }
}
