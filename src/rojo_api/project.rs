use std::path::PathBuf;

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

        let path: Option<String> = lua_table.get("$path")?;
        let path = path.map(|value| PathBuf::from(value));

        let class_name: Option<String> = lua_table.get("$className")?;

        let project_node = ProjectNode {
            path,
            class_name,
            properties: Default::default(),
            ignore_unknown_instances: None,
            children: Default::default(),
        };

        Ok(RemodelProjectNode(project_node))
    }
}
