use std::{
    collections::VecDeque,
    iter::FromIterator,
    sync::{Arc, Mutex},
};

use mlua::{FromLua, Lua, MetaMethod, ToLua, UserData, UserDataMethods};
use rbx_dom_weak::{types::Ref, InstanceBuilder, WeakDom};
use rbx_reflection::ClassTag;

#[derive(Clone)]
pub struct LuaInstance {
    pub tree: Arc<Mutex<WeakDom>>,
    pub id: Ref,
}

impl LuaInstance {
    pub fn new(tree: Arc<Mutex<WeakDom>>, id: Ref) -> Self {
        LuaInstance { tree, id }
    }

    fn clear_all_children(&self) -> mlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call ClearAllChildren() on a destroyed instance")
        })?;

        let children: Vec<Ref> = instance.children().to_vec();

        for child_id in children {
            tree.destroy(child_id);
        }

        Ok(())
    }

    fn clone_instance(&self) -> mlua::Result<LuaInstance> {
        let mut tree = self.tree.lock().unwrap();

        if tree.get_by_ref(self.id).is_none() {
            return Err(mlua::Error::external(
                "Cannot call Clone() on a destroyed instance",
            ));
        }

        let root_id = tree.root_ref();
        let new_id = Self::clone_kernel(&mut tree, self.id, root_id);

        Ok(LuaInstance {
            tree: Arc::clone(&self.tree),
            id: new_id,
        })
    }

    fn clone_kernel(tree: &mut WeakDom, id: Ref, parent_id: Ref) -> Ref {
        let instance = tree.get_by_ref(id).unwrap();
        let builder = InstanceBuilder::new(&instance.class)
            .with_name(&instance.name)
            .with_properties(
                instance
                    .properties
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone())),
            );
        let children = instance.children().to_vec();

        let new_id = tree.insert(parent_id, builder);

        for child_id in children {
            Self::clone_kernel(tree, child_id, new_id);
        }

        new_id
    }

    fn destroy(&self) -> mlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        if tree.get_by_ref(self.id).is_none() {
            return Err(mlua::Error::external(
                "Cannot call Destroy() on a destroyed instance",
            ));
        }

        tree.destroy(self.id);

        Ok(())
    }

    fn find_first_child(&self, name: &str) -> mlua::Result<Option<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call FindFirstChild() on a destroyed instance")
        })?;

        let child = instance
            .children()
            .iter()
            .copied()
            .find(|id| {
                if let Some(child_instance) = tree.get_by_ref(*id) {
                    return child_instance.name == name;
                }

                false
            })
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), id));

        Ok(child)
    }

    fn find_first_child_of_class(&self, class_name: &str) -> mlua::Result<Option<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call FindFirstChildOfClass() on a destroyed instance")
        })?;

        let child = instance
            .children()
            .iter()
            .copied()
            .find(|id| {
                if let Some(child_instance) = tree.get_by_ref(*id) {
                    return child_instance.class == class_name;
                }

                false
            })
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), id));

        Ok(child)
    }

    fn get_full_name(&self) -> mlua::Result<String> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call GetFullName() on a destroyed instance")
        })?;

        let mut names = vec![instance.name.as_str()];
        let mut current = instance.parent();

        while let Some(parent_instance) = tree.get_by_ref(current) {
            if current != tree.root_ref() && parent_instance.class != "DataModel" {
                names.push(parent_instance.name.as_str());
            }
            current = parent_instance.parent();
        }

        names.reverse();

        Ok(names.join("."))
    }

    fn get_descendants(&self) -> mlua::Result<Vec<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call GetDescendants() on a destroyed instance")
        })?;

        let mut descendants = Vec::new();
        let mut stack = VecDeque::from_iter(instance.children().into_iter());

        while let Some(current) = stack.pop_front() {
            descendants.push(LuaInstance::new(Arc::clone(&self.tree), *current));

            let current_instance = tree
                .get_by_ref(*current)
                .expect("received invalid child in tree when recursing through descendants");

            for child in current_instance.children().iter().rev() {
                stack.push_front(child);
            }
        }

        Ok(descendants)
    }

    fn get_children(&self) -> mlua::Result<Vec<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call GetChildren() on a destroyed instance")
        })?;

        let children: Vec<LuaInstance> = instance
            .children()
            .iter()
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), *id))
            .collect();

        Ok(children)
    }

    fn get_service(&self, service_name: &str) -> mlua::Result<LuaInstance> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot call GetService() on a destroyed instance")
        })?;

        // It might be cleaner to avoid defining GetService() on all instances,
        // but we don't have a good mechanism in Remodel to do that right now.
        if instance.class != "DataModel" {
            return Err(mlua::Error::external(
                "Cannot call GetService() on an instance that is not a DataModel",
            ));
        }

        let database = rbx_reflection_database::get();

        match database.classes.get(service_name) {
            // We should only find services, even if there's a child of
            // DataModel with a matching ClassName.
            Some(descriptor) if descriptor.tags.contains(&ClassTag::Service) => {
                let existing = instance
                    .children()
                    .iter()
                    .copied()
                    .map(|id| tree.get_by_ref(id).unwrap())
                    .find(|instance| instance.class == service_name);

                match existing {
                    Some(existing) => Ok(LuaInstance {
                        tree: Arc::clone(&self.tree),
                        id: existing.referent(),
                    }),
                    None => {
                        // If we didn't find an existing service instance,
                        // construct a new one.

                        let properties = InstanceBuilder::new(service_name);

                        let id = tree.insert(self.id, properties);
                        Ok(LuaInstance {
                            tree: Arc::clone(&self.tree),
                            id,
                        })
                    }
                }
            }
            _ => Err(mlua::Error::external(format!(
                "'{}' is not a valid service.",
                service_name
            ))),
        }
    }

    fn get_class_name<'lua>(&self, context: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot access ClassName on a destroyed instance")
        })?;

        instance.class.as_str().to_lua(context)
    }

    fn get_name<'lua>(&self, context: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_by_ref(self.id)
            .ok_or_else(|| mlua::Error::external("Cannot access Name on a destroyed instance"))?;

        instance.name.as_str().to_lua(context)
    }

    fn set_name(&self, value: mlua::Value<'_>) -> mlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree
            .get_by_ref_mut(self.id)
            .ok_or_else(|| mlua::Error::external("Cannot set Name on a destroyed instance"))?;

        match value {
            mlua::Value::String(lua_str) => {
                instance.name = lua_str.to_str()?.to_string();

                Ok(())
            }
            _ => Err(mlua::Error::external("Instance.Name must be a string")),
        }
    }

    fn get_parent<'lua>(&self, context: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_by_ref(self.id)
            .ok_or_else(|| mlua::Error::external("Cannot access Parent on a destroyed instance"))?;

        match instance.parent() {
            parent if parent.is_some() => {
                if parent == tree.root_ref() {
                    Ok(mlua::Value::Nil)
                } else {
                    LuaInstance::new(Arc::clone(&self.tree), parent).to_lua(context)
                }
            }
            _nil => Ok(mlua::Value::Nil),
        }
    }

    fn set_parent<'lua>(&self, context: &'lua Lua, value: mlua::Value<'lua>) -> mlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        match Option::<LuaInstance>::from_lua(value, context)? {
            Some(new_parent) => {
                tree.transfer_within(self.id, new_parent.id);
            }
            None => {
                let root_id = tree.root_ref();
                tree.transfer_within(self.id, root_id);
            }
        }

        Ok(())
    }

    fn get_property<'lua>(
        &self,
        _context: &'lua Lua,
        _name: &str,
    ) -> mlua::Result<Option<mlua::Value<'lua>>> {
        // TODO: Use rbx_reflection to look up property descriptors
        Ok(None)
    }

    fn meta_to_string<'lua>(&self, context: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            mlua::Error::external("Cannot invoke tostring on a destroyed instance")
        })?;

        instance.name.as_str().to_lua(context)
    }

    fn meta_index<'lua>(&self, context: &'lua Lua, key: &str) -> mlua::Result<mlua::Value<'lua>> {
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

                Err(mlua::Error::external(format!(
                    "'{}' is not a valid member of Instance",
                    key
                )))
            }
        }
    }

    fn meta_new_index<'lua>(
        &self,
        context: &'lua Lua,
        key: &str,
        value: mlua::Value<'lua>,
    ) -> mlua::Result<()> {
        match key {
            "Name" => self.set_name(value),
            "ClassName" => Err(mlua::Error::external("Instance.ClassName is read-only.")),
            "Parent" => self.set_parent(context, value),

            // Setting unknown keys is an error.
            _ => Err(mlua::Error::external(format!(
                "'{}' is not a valid member of Instance",
                key
            ))),
        }
    }
}

impl UserData for LuaInstance {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("ClearAllChildren", |_context, this, _args: ()| {
            this.clear_all_children()
        });

        methods.add_method("Clone", |_context, this, _args: ()| this.clone_instance());

        methods.add_method("Destroy", |_context, this, _args: ()| this.destroy());

        methods.add_method("FindFirstChild", |_context, this, name: String| {
            this.find_first_child(&name)
        });

        methods.add_method("FindFirstChildOfClass", |_context, this, class: String| {
            this.find_first_child_of_class(&class)
        });

        methods.add_method("GetFullName", |_context, this, _args: ()| {
            this.get_full_name()
        });

        methods.add_method("GetChildren", |_context, this, _args: ()| {
            this.get_children()
        });

        methods.add_method("GetDescendants", |_context, this, _args: ()| {
            this.get_descendants()
        });

        methods.add_method("GetService", |_context, this, name: String| {
            this.get_service(&name)
        });

        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.meta_to_string(context)
        });

        methods.add_meta_method(MetaMethod::Index, |context, this, key: String| {
            this.meta_index(context, &key)
        });

        methods.add_meta_method(
            MetaMethod::NewIndex,
            |context, this, (key, value): (String, mlua::Value)| {
                this.meta_new_index(context, &key, value)
            },
        );

        methods.add_meta_function(
            MetaMethod::Eq,
            |_context, (a, b): (LuaInstance, LuaInstance)| Ok(a.id == b.id),
        );
    }
}
