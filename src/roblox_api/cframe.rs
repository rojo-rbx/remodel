use mlua::{UserData, UserDataMethods, Value as LuaValue};
use rbx_dom_weak::types::{CFrame, Matrix3, Vector3};

use crate::value::{CFrameValue, Vector3Value};

pub struct CFrameUserData;

impl CFrameUserData {
    fn from_position(x: f32, y: f32, z: f32) -> CFrameValue {
        CFrameValue::new(CFrame::new(Vector3::new(x, y, z), Matrix3::identity()))
    }
}

fn try_into_f32(value: LuaValue<'_>) -> Option<f32> {
    match value {
        LuaValue::Number(num) => Some(num as f32),
        LuaValue::Integer(int) => Some(int as f32),
        _ => None,
    }
}

impl UserData for CFrameUserData {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "new",
            |_context,
             arguments: (
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
            )| {
                match arguments {
                    (None, None, None) => return Ok(Self::from_position(0.0, 0.0, 0.0)),
                    (Some(LuaValue::UserData(user_data)), None, None) => {
                        let position = &*user_data.borrow::<Vector3Value>()?;
                        return Ok(CFrameValue::new(CFrame::new(
                            position.inner(),
                            Matrix3::identity(),
                        )));
                    }
                    _ => {}
                };

                let x = arguments.0.and_then(try_into_f32);
                let y = arguments.1.and_then(try_into_f32);
                let z = arguments.2.and_then(try_into_f32);

                match (x, y, z) {
                    (Some(x), Some(y), Some(z)) => Ok(Self::from_position(x, y, z)),
                    _ => Err(mlua::Error::external(
                        "invalid argument #1 to 'new' (Vector3 expected)",
                    )),
                }
            },
        );
    }
}
