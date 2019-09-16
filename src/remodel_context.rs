//! The state global to a given Lua state is stored in the Lua registry inside
//! `RemodelContext`, defined by this module.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree};
use rlua::{Context, UserData};

#[derive(Clone)]
pub struct RemodelContext {
    pub master_tree: Arc<Mutex<RbxTree>>,
}

impl RemodelContext {
    pub fn get<'lua>(context: Context<'lua>) -> rlua::Result<Self> {
        context.named_registry_value("remodel_context")
    }

    pub fn inject<'lua>(context: Context<'lua>) -> rlua::Result<()> {
        let master_tree = Arc::new(Mutex::new(RbxTree::new(RbxInstanceProperties {
            name: "REMODEL ROOT".to_owned(),
            class_name: "REMODEL ROOT".to_owned(),
            properties: HashMap::new(),
        })));

        let remodel_context = RemodelContext { master_tree };

        context.set_named_registry_value("remodel_context", remodel_context)?;

        Ok(())
    }
}

impl UserData for RemodelContext {}
