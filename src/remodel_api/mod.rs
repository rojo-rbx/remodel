mod json;
mod remodel;

use mlua::{FromLuaMulti, Lua, Value as LuaValue};

pub use json::Json;
pub use remodel::Remodel;

use crate::roblox_api::LuaInstance;

pub struct RemodelApi;

impl RemodelApi {
    pub fn inject(context: &Lua) -> mlua::Result<()> {
        context.globals().set("remodel", Remodel)?;
        context.globals().set("json", Json)?;

        Ok(())
    }
}

/// Takes (String, LuaInstance) as a variadic, but reports a useful error
/// if the arguments are swapped.
pub struct StringLuaInstanceBackCompat(pub (String, LuaInstance));

impl<'lua> FromLuaMulti<'lua> for StringLuaInstanceBackCompat {
    fn from_lua_multi(values: mlua::MultiValue<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
        let values = values.into_vec();

        match (values.get(0), values.get(1)) {
            (Some(LuaValue::String(filename)), Some(LuaValue::UserData(user_data))) => {
                let instance = user_data.take::<LuaInstance>()?;

                Ok(StringLuaInstanceBackCompat((
                    filename.to_str()?.to_owned(),
                    instance,
                )))
            }

            (Some(user_data_value @ LuaValue::UserData(user_data)), Some(LuaValue::String(_))) => {
                if user_data.is::<LuaInstance>() {
                    Err(mlua::Error::external("The two arguments are swapped. The first argument should be the filename, and the second argument should be the instance."))
                } else {
                    Err(mlua::Error::external(format!(
                        "Expected two arguments, a filename and an Instance. received string and {}.",
                        user_data_value.type_name()
                    )))
                }
            }

            (Some(one), Some(two)) => Err(mlua::Error::external(format!(
                "Expected two arguments, a filename and an Instance. received {} and {}.",
                one.type_name(),
                two.type_name()
            ))),

            (Some(one), None) => Err(mlua::Error::external(format!(
                "Expected two arguments, a filename and an Instance. only received {}.",
                one.type_name()
            ))),

            (None, Some(_)) => {
                unreachable!("the first value of the MultiValue was None, but the second was Some")
            }

            (None, None) => Err(mlua::Error::external(
                "Expected two arguments, a filename and an Instance.",
            )),
        }
    }
}
