use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use librojo::project::{Project, ProjectNode};
use rbx_dom_weak::{AmbiguousRbxValue, RbxValue, UnresolvedRbxValue};
use rlua::{Context, FromLua, Value};

pub struct RemodelProject(pub Project);

impl<'lua> FromLua<'lua> for RemodelProject {
    fn from_lua(lua_value: Value<'lua>, _context: Context<'lua>) -> rlua::Result<Self> {
        let lua_table = match lua_value {
            Value::Table(table) => table,
            _ => {
                return Err(rlua::Error::FromLuaConversionError {
                    from: "not a table",
                    to: "table",
                    message: None,
                })
            }
        };

        let name: String = lua_table.get("name")?;
        let tree: RemodelProjectNode = lua_table.get("tree")?;

        let file_location: String = lua_table.get("fileLocation")?;
        let file_location = PathBuf::from(file_location);

        let project = Project {
            name,
            tree: tree.0,
            serve_port: None,
            serve_place_ids: None,
            file_location,
        };

        Ok(RemodelProject(project))
    }
}

struct RemodelProjectNode(ProjectNode);

impl<'lua> FromLua<'lua> for RemodelProjectNode {
    fn from_lua(lua_value: Value<'lua>, context: Context<'lua>) -> rlua::Result<Self> {
        let lua_table = match lua_value {
            Value::Table(table) => table,
            _ => {
                return Err(rlua::Error::FromLuaConversionError {
                    from: "not a table",
                    to: "table",
                    message: None,
                })
            }
        };

        let mut path = None;
        let mut class_name = None;
        let mut ignore_unknown_instances = None;
        let mut children = BTreeMap::new();
        let mut properties: HashMap<String, RemodelUnresolvedValue> = HashMap::new();

        for pair in lua_table.pairs::<String, Value<'_>>() {
            let (key, value) = pair?;

            match key.as_str() {
                "$path" => path = Some(String::from_lua(value, context)?),
                "$className" => class_name = Some(String::from_lua(value, context)?),
                "$ignoreUnknownInstances" => {
                    ignore_unknown_instances = Some(bool::from_lua(value, context)?)
                }
                "$properties" => {
                    properties = HashMap::from_lua(value, context)?;
                }
                _ => {
                    children.insert(key, RemodelProjectNode::from_lua(value, context)?.0);
                }
            }
        }

        let path = path.map(|value| PathBuf::from(value));

        // Unwrap all of our Remodel values into values that Rojo understands.
        let properties = properties
            .into_iter()
            .map(|(key, value)| (key, value.0))
            .collect();

        let project_node = ProjectNode {
            path,
            class_name,
            children,
            properties,
            ignore_unknown_instances,
        };

        Ok(RemodelProjectNode(project_node))
    }
}

struct RemodelUnresolvedValue(UnresolvedRbxValue);

impl<'lua> FromLua<'lua> for RemodelUnresolvedValue {
    fn from_lua(lua_value: Value<'lua>, context: Context<'lua>) -> rlua::Result<Self> {
        let value = lua_to_unresolved_value(lua_value, context)?;
        Ok(RemodelUnresolvedValue(value))
    }
}

fn lua_to_unresolved_value<'lua>(
    lua_value: Value<'lua>,
    context: Context<'lua>,
) -> rlua::Result<UnresolvedRbxValue> {
    match lua_value {
        Value::String(value) => Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::String(
            value.to_str()?.to_owned(),
        ))),
        Value::Boolean(value) => Ok(UnresolvedRbxValue::Concrete(RbxValue::Bool { value })),
        Value::Number(num) => Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(
            num,
        ))),
        Value::Integer(num) => Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float1(
            num as f64,
        ))),
        Value::Table(table) => {
            let ty = table.raw_get("Type")?;
            let value = table.raw_get("Value")?;

            match (ty, value) {
                (Value::Nil, Value::Nil) => {
                    // When neither field is defined, we assume this is an
                    // array-like table specifying an ambiguous value.

                    let values = Vec::<f64>::from_lua(Value::Table(table), context)?;

                    match values.as_slice() {
                        [a, b] => Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float2(
                            *a, *b,
                        ))),
                        [a, b, c] => Ok(UnresolvedRbxValue::Ambiguous(AmbiguousRbxValue::Float3(
                            *a, *b, *c,
                        ))),
                        [a, b, c, d] => Ok(UnresolvedRbxValue::Concrete(RbxValue::UDim2 {
                            value: (*a as f32, *b as i32, *c as f32, *d as i32),
                        })),
                        [a, b, c, d, e, f, g, h, i, j, k, l] => {
                            Ok(UnresolvedRbxValue::Concrete(RbxValue::CFrame {
                                value: [
                                    *a as f32, *b as f32, *c as f32, *d as f32, *e as f32,
                                    *f as f32, *g as f32, *h as f32, *i as f32, *j as f32,
                                    *k as f32, *l as f32,
                                ],
                            }))
                        }
                        _ => {
                            return Err(rlua::Error::FromLuaConversionError {
                                from: "not a Roblox value",
                                to: "Roblox value",
                                message: None,
                            });
                        }
                    }
                }
                (Value::String(_ty_name), _not_nil) => {
                    unimplemented!();
                }
                _ => {
                    return Err(rlua::Error::FromLuaConversionError {
                        from: "not a Roblox value",
                        to: "Roblox value",
                        message: None,
                    });
                }
            }
        }
        _ => {
            return Err(rlua::Error::FromLuaConversionError {
                from: "not a Roblox value",
                to: "Roblox value",
                message: None,
            });
        }
    }
}
