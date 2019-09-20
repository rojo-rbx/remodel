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
    auth_cookie: Option<String>,
}

impl RemodelContext {
    pub fn new(auth_cookie: Option<String>) -> Self {
        let master_tree = Arc::new(Mutex::new(RbxTree::new(RbxInstanceProperties {
            name: "REMODEL ROOT".to_owned(),
            class_name: "REMODEL ROOT".to_owned(),
            properties: HashMap::new(),
        })));

        Self {
            master_tree,
            auth_cookie,
        }
    }

    pub fn get(context: Context<'_>) -> rlua::Result<Self> {
        context.named_registry_value("remodel_context")
    }

    pub fn inject(self, context: Context<'_>) -> rlua::Result<()> {
        context.set_named_registry_value("remodel_context", self)?;

        Ok(())
    }

    pub fn auth_cookie(&self) -> Option<&str> {
        self.auth_cookie.as_ref().map(|v| v.as_str())
    }
}

impl UserData for RemodelContext {}
