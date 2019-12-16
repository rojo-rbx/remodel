use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
};

use librojo::project::{Project, ProjectNode};
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
        let properties = HashMap::new();

        for pair in lua_table.pairs::<String, Value<'_>>() {
            let (key, value) = pair?;

            match key.as_str() {
                "$path" => path = Some(String::from_lua(value, context)?),
                "$className" => class_name = Some(String::from_lua(value, context)?),
                "$ignoreUnknownInstances" => {
                    ignore_unknown_instances = Some(bool::from_lua(value, context)?)
                }
                "$properties" => {
                    return Err(rlua::Error::external("$properties is not implemented yet"));
                }
                _ => {
                    children.insert(key, RemodelProjectNode::from_lua(value, context)?.0);
                }
            }
        }

        let path = path.map(|value| PathBuf::from(value));

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
