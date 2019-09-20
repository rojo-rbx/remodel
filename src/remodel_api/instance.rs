use std::sync::{Arc, Mutex};

use rbx_dom_weak::{RbxId, RbxTree};
use rlua::{Context, FromLua, MetaMethod, ToLua, UserData, UserDataMethods};

#[derive(Clone)]
pub struct LuaInstance {
    pub tree: Arc<Mutex<RbxTree>>,
    pub id: RbxId,
}

impl LuaInstance {
    pub fn new(tree: Arc<Mutex<RbxTree>>, id: RbxId) -> Self {
        LuaInstance { tree, id }
    }

    fn find_first_child(&self, name: &str) -> rlua::Result<Option<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        let child = instance
            .get_children_ids()
            .into_iter()
            .copied()
            .find(|id| {
                if let Some(child_instance) = tree.get_instance(*id) {
                    return child_instance.name == name;
                }

                return false;
            })
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), id));

        Ok(child)
    }

    fn get_children(&self) -> rlua::Result<Vec<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        let children: Vec<LuaInstance> = instance
            .get_children_ids()
            .into_iter()
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), *id))
            .collect();

        Ok(children)
    }

    fn get_class_name<'lua>(
        &self,
        context: rlua::Context<'lua>,
    ) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        instance.class_name.as_str().to_lua(context)
    }

    fn get_name<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        instance.name.as_str().to_lua(context)
    }

    fn set_name(&self, value: rlua::Value<'_>) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance_mut(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        match value {
            rlua::Value::String(lua_str) => {
                instance.name = lua_str.to_str()?.to_string();

                Ok(())
            }
            _ => Err(rlua::Error::external(format!("'Name' must be a string."))),
        }
    }

    fn get_parent<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        match instance.get_parent_id() {
            Some(parent_id) => {
                if parent_id == tree.get_root_id() {
                    Ok(rlua::Value::Nil)
                } else {
                    LuaInstance::new(Arc::clone(&self.tree), parent_id).to_lua(context)
                }
            }
            None => Ok(rlua::Value::Nil),
        }
    }

    fn set_parent<'lua>(
        &self,
        context: Context<'lua>,
        value: rlua::Value<'lua>,
    ) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        match Option::<LuaInstance>::from_lua(value, context)? {
            Some(new_parent) => {
                tree.set_parent(self.id, new_parent.id);
            }
            None => {
                let root_id = tree.get_root_id();
                tree.set_parent(self.id, root_id);
            }
        }

        Ok(())
    }

    fn get_property<'lua>(
        &self,
        _context: Context<'lua>,
        _name: &str,
    ) -> rlua::Result<Option<rlua::Value<'lua>>> {
        // TODO: Use rbx_reflection to look up property descriptors
        Ok(None)
    }

    fn meta_to_string<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Instance was destroyed"))?;

        instance.name.as_str().to_lua(context)
    }

    fn meta_index<'lua>(
        &self,
        context: Context<'lua>,
        key: &str,
    ) -> rlua::Result<rlua::Value<'lua>> {
        match key {
            "Name" => self.get_name(context),
            "ClassName" => self.get_class_name(context),
            "Parent" => self.get_parent(context),

            // Getting an unknown key falls back to properties, then children.
            _ => {
                if let Some(value) = self.get_property(context, key)? {
                    return Ok(value);
                }

                if let Some(child) = self.find_first_child(key)? {
                    return child.to_lua(context);
                }

                Err(rlua::Error::external(format!(
                    "'{}' is not a valid member of Instance",
                    key
                )))
            }
        }
    }

    fn meta_new_index<'lua>(
        &self,
        context: Context<'lua>,
        key: &str,
        value: rlua::Value<'lua>,
    ) -> rlua::Result<()> {
        match key {
            "Name" => self.set_name(value),
            "ClassName" => Err(rlua::Error::external("'ClassName' is read-only.")),
            "Parent" => self.set_parent(context, value),

            // Setting unknown keys is an error.
            _ => Err(rlua::Error::external(format!(
                "'{}' is not a valid member of Instance",
                key
            ))),
        }
    }
}

impl UserData for LuaInstance {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("FindFirstChild", |_context, this, name: String| {
            this.find_first_child(&name)
        });

        methods.add_method("GetChildren", |_context, this, _args: ()| {
            this.get_children()
        });

        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.meta_to_string(context)
        });

        methods.add_meta_method(MetaMethod::Index, |context, this, key: String| {
            this.meta_index(context, &key)
        });

        methods.add_meta_method(
            MetaMethod::NewIndex,
            |context, this, (key, value): (String, rlua::Value)| {
                this.meta_new_index(context, &key, value)
            },
        );

        methods.add_meta_function(
            MetaMethod::Eq,
            |_context, (a, b): (LuaInstance, LuaInstance)| Ok(a.id == b.id),
        );
    }
}
