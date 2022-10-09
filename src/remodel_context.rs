//! The state global to a given Lua state is stored in the Lua registry inside
//! `RemodelContext`, defined by this module.

use std::sync::{Arc, Mutex};

use mlua::{Lua, UserData};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

#[derive(Clone)]
pub struct RemodelContext {
    pub master_tree: Arc<Mutex<WeakDom>>,
    auth_cookie: Option<String>,
    api_key: Option<String>,
}

impl RemodelContext {
    pub fn new(auth_cookie: Option<String>, api_key: Option<String>) -> Self {
        let master_tree = Arc::new(Mutex::new(WeakDom::new(InstanceBuilder::new(
            "RemodelRoot",
        ))));

        Self {
            master_tree,
            auth_cookie,
            api_key,
        }
    }

    pub fn get(context: &Lua) -> mlua::Result<Self> {
        context.named_registry_value("remodel_context")
    }

    pub fn inject(self, context: &Lua) -> mlua::Result<()> {
        context.set_named_registry_value("remodel_context", self)?;

        Ok(())
    }

    pub fn auth_cookie(&self) -> Option<&str> {
        self.auth_cookie.as_ref().map(|v| v.as_str())
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_ref().map(|v| v.as_str())
    }
}

impl UserData for RemodelContext {}
