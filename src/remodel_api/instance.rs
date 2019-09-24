use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

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

    fn clone_instance(&self) -> rlua::Result<LuaInstance> {
        let mut tree = self.tree.lock().unwrap();

        if tree.get_instance(self.id).is_none() {
            return Err(rlua::Error::external(
                "Cannot call Clone() on a destroyed instance",
            ));
        }

        let root_id = tree.get_root_id();
        let new_id = Self::clone_kernel(&mut tree, self.id, root_id);

        Ok(LuaInstance {
            tree: Arc::clone(&self.tree),
            id: new_id,
        })
    }

    fn clone_kernel(tree: &mut RbxTree, id: RbxId, parent_id: RbxId) -> RbxId {
        let instance = tree.get_instance(id).unwrap();
        let properties = instance.deref().clone();
        let children = instance.get_children_ids().to_vec();

        let new_id = tree.insert_instance(properties, parent_id);

        for child_id in children {
            Self::clone_kernel(tree, child_id, new_id);
        }

        new_id
    }

    fn destroy(&self) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        // TODO: https://github.com/rojo-rbx/rbx-dom/issues/75
        // This check is necessary because RbxTree::remove_instance panics if
        // the input ID doesn't exist instead of returning None.
        if tree.get_instance(self.id).is_none() {
            return Err(rlua::Error::external(
                "Cannot call Destroy() on a destroyed instance",
            ));
        }

        if tree.remove_instance(self.id).is_none() {
            return Err(rlua::Error::external(
                "Cannot call Destroy() on a destroyed instance",
            ));
        }

        Ok(())
    }

    fn find_first_child(&self, name: &str) -> rlua::Result<Option<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_instance(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call FindFirstChild() on a destroyed instance")
        })?;

        let child = instance
            .get_children_ids()
            .iter()
            .copied()
            .find(|id| {
                if let Some(child_instance) = tree.get_instance(*id) {
                    return child_instance.name == name;
                }

                false
            })
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), id));

        Ok(child)
    }

    fn get_children(&self) -> rlua::Result<Vec<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_instance(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call GetChildren() on a destroyed instance")
        })?;

        let children: Vec<LuaInstance> = instance
            .get_children_ids()
            .iter()
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), *id))
            .collect();

        Ok(children)
    }

    fn get_class_name<'lua>(
        &self,
        context: rlua::Context<'lua>,
    ) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_instance(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot access ClassName on a destroyed instance")
        })?;

        instance.class_name.as_str().to_lua(context)
    }

    fn get_name<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Cannot access Name on a destroyed instance"))?;

        instance.name.as_str().to_lua(context)
    }

    fn set_name(&self, value: rlua::Value<'_>) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance_mut(self.id)
            .ok_or_else(|| rlua::Error::external("Cannot set Name on a destroyed instance"))?;

        match value {
            rlua::Value::String(lua_str) => {
                instance.name = lua_str.to_str()?.to_string();

                Ok(())
            }
            _ => Err(rlua::Error::external("Instance.Name must be a string")),
        }
    }

    fn get_parent<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_instance(self.id)
            .ok_or_else(|| rlua::Error::external("Cannot access Parent on a destroyed instance"))?;

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

        let instance = tree.get_instance(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot invoke tostring on a destroyed instance")
        })?;

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
            "ClassName" => Err(rlua::Error::external("Instance.ClassName is read-only.")),
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
        methods.add_method("Clone", |_context, this, _args: ()| this.clone_instance());

        methods.add_method("Destroy", |_context, this, _args: ()| this.destroy());

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
