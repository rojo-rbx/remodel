use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rbx_dom_weak::{
    types::{attributes_from_map, get_attributes, BinaryString, Ref, Variant, VariantType},
    InstanceBuilder, WeakDom,
};
use rbx_reflection::ClassTag;
use rlua::{Context, FromLua, MetaMethod, ToLua, UserData, UserDataMethods};

use crate::value::{guess_type_from_rbxvalue, lua_to_rbxvalue, rbxvalue_to_lua};

#[derive(Clone)]
pub struct LuaInstance {
    pub tree: Arc<Mutex<WeakDom>>,
    pub id: Ref,
}

impl LuaInstance {
    pub fn new(tree: Arc<Mutex<WeakDom>>, id: Ref) -> Self {
        LuaInstance { tree, id }
    }

    fn clone_instance(&self) -> rlua::Result<LuaInstance> {
        let mut tree = self.tree.lock().unwrap();

        if tree.get_by_ref(self.id).is_none() {
            return Err(rlua::Error::external(
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

    fn destroy(&self) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        if tree.get_by_ref(self.id).is_none() {
            return Err(rlua::Error::external(
                "Cannot call Destroy() on a destroyed instance",
            ));
        }

        tree.destroy(self.id);

        Ok(())
    }

    fn find_first_child(&self, name: &str) -> rlua::Result<Option<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call FindFirstChild() on a destroyed instance")
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

    fn get_attribute<'lua>(
        &self,
        context: rlua::Context<'lua>,
        attribute_name: &str,
    ) -> rlua::Result<Option<rlua::Value<'lua>>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call GetAttribute() on a destroyed instance")
        })?;

        let attribute_binary_string = match instance.properties.get("AttributesSerialize") {
            Some(Variant::BinaryString(bytes)) => bytes,
            Some(other) => {
                return Err(rlua::Error::external(format!(
                    "AttributesSerialize is `{:?}`, not a BinaryString",
                    other,
                )))
            }
            None => return Ok(None),
        };

        let attribute_bytes: &[u8] = attribute_binary_string.as_ref();
        let mut attributes = get_attributes(attribute_bytes).map_err(|error| {
            rlua::Error::external(format!("Attributes could not be deserialized: {}", error))
        })?;

        rbxvalue_to_lua(
            context,
            &match attributes.remove(attribute_name.as_bytes()) {
                Some(variant) => match variant {
                    Variant::BinaryString(binary) => {
                        Variant::String(String::from_utf8_lossy(binary.as_ref()).into_owned())
                    }
                    other => other,
                },
                None => return Ok(None),
            },
        )
        .map(Some)
    }

    fn set_attribute<'lua>(
        &self,
        context: rlua::Context<'lua>,
        attribute_name: &str,
        value: rlua::Value<'lua>,
    ) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref_mut(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call GetAttribute() on a destroyed instance")
        })?;

        let mut attributes = match instance.properties.get("AttributesSerialize") {
            Some(Variant::BinaryString(bytes)) => {
                let attribute_bytes: &[u8] = bytes.as_ref();
                get_attributes(attribute_bytes).map_err(|error| {
                    rlua::Error::external(format!(
                        "Attributes could not be deserialized: {}",
                        error
                    ))
                })?
            }
            Some(other) => {
                return Err(rlua::Error::external(format!(
                    "AttributesSerialize is `{:?}`, not a BinaryString",
                    other,
                )))
            }
            None => HashMap::new(),
        };

        let attribute_key = attribute_name.as_bytes().to_vec();

        if matches!(value, rlua::Value::Nil) {
            attributes.remove(&attribute_key);
        } else {
            let guessed_type = guess_type_from_rbxvalue(&value).ok_or_else(|| {
                rlua::Error::external(format!("{:?} is not a valid attribute value", value))
            })?;

            let (variant_type, value) = match (guessed_type, value) {
                (VariantType::String, value) => (
                    VariantType::BinaryString,
                    // I think I made a mistake with base64 :/
                    rlua::Value::String(context.create_string(&base64::encode(match value {
                        rlua::Value::String(string) => string,
                        _ => unreachable!(),
                    }))?),
                ),

                other @ _ => other,
            };

            attributes.insert(attribute_key, lua_to_rbxvalue(variant_type, value)?);
        }

        instance.properties.insert(
            "AttributesSerialize".to_owned(),
            Variant::BinaryString(BinaryString::from(
                attributes_from_map(attributes).map_err(|error| {
                    rlua::Error::external(format!(
                        "There was an error while setting the attribute: {}",
                        error,
                    ))
                })?,
            )),
        );

        Ok(())
    }

    fn get_children(&self) -> rlua::Result<Vec<LuaInstance>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call GetChildren() on a destroyed instance")
        })?;

        let children: Vec<LuaInstance> = instance
            .children()
            .iter()
            .map(|id| LuaInstance::new(Arc::clone(&self.tree), *id))
            .collect();

        Ok(children)
    }

    fn get_service(&self, service_name: &str) -> rlua::Result<LuaInstance> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot call GetService() on a destroyed instance")
        })?;

        // It might be cleaner to avoid defining GetService() on all instances,
        // but we don't have a good mechanism in Remodel to do that right now.
        if instance.class != "DataModel" {
            return Err(rlua::Error::external(
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
            _ => Err(rlua::Error::external(format!(
                "'{}' is not a valid service.",
                service_name
            ))),
        }
    }

    fn get_class_name<'lua>(
        &self,
        context: rlua::Context<'lua>,
    ) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
            rlua::Error::external("Cannot access ClassName on a destroyed instance")
        })?;

        instance.class.as_str().to_lua(context)
    }

    fn get_name<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree
            .get_by_ref(self.id)
            .ok_or_else(|| rlua::Error::external("Cannot access Name on a destroyed instance"))?;

        instance.name.as_str().to_lua(context)
    }

    fn set_name(&self, value: rlua::Value<'_>) -> rlua::Result<()> {
        let mut tree = self.tree.lock().unwrap();

        let instance = tree
            .get_by_ref_mut(self.id)
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
            .get_by_ref(self.id)
            .ok_or_else(|| rlua::Error::external("Cannot access Parent on a destroyed instance"))?;

        match instance.parent() {
            parent if parent.is_some() => {
                if parent == tree.root_ref() {
                    Ok(rlua::Value::Nil)
                } else {
                    LuaInstance::new(Arc::clone(&self.tree), parent).to_lua(context)
                }
            }
            _nil => Ok(rlua::Value::Nil),
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
        _context: Context<'lua>,
        _name: &str,
    ) -> rlua::Result<Option<rlua::Value<'lua>>> {
        // TODO: Use rbx_reflection to look up property descriptors
        Ok(None)
    }

    fn meta_to_string<'lua>(&self, context: Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        let tree = self.tree.lock().unwrap();

        let instance = tree.get_by_ref(self.id).ok_or_else(|| {
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

        methods.add_method("GetAttribute", |context, this, attribute: String| {
            this.get_attribute(context, &attribute)
        });

        methods.add_method("GetChildren", |_context, this, _args: ()| {
            this.get_children()
        });

        methods.add_method("GetService", |_context, this, name: String| {
            this.get_service(&name)
        });

        methods.add_method(
            "SetAttribute",
            |context, this, (attribute, value): (String, rlua::Value<'_>)| {
                this.set_attribute(context, &attribute, value)
            },
        );

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
