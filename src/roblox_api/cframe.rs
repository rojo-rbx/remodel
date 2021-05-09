use rbx_dom_weak::types::{CFrame as DomCFrame, Matrix3, Vector3};
use rlua::{UserData, UserDataMethods, Value as LuaValue};

use crate::value::{CFrameValue, Vector3Value};

pub struct CFrame;

impl CFrame {
    fn from_position(x: f32, y: f32, z: f32) -> CFrameValue {
        CFrameValue::new(DomCFrame::new(
            Vector3::new(x as f32, y as f32, z as f32),
            // TODO: replace with `Matrix3::identity()` once
            // a version higher than 0.3.0 of rbx_types ships
            Matrix3::new(
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ),
        ))
    }
}

#[derive(Debug, Clone)]
enum Either {
    Number(f32),
    Vector(Vector3Value),
    Other,
}

impl From<LuaValue<'_>> for Either {
    fn from(value: LuaValue<'_>) -> Self {
        match value {
            LuaValue::Number(number) => Self::Number(number as f32),
            LuaValue::Integer(number) => Self::Number(number as f32),
            LuaValue::UserData(user_data) => user_data
                .borrow::<Vector3Value>()
                .ok()
                .map(|vector| Self::Vector(*vector))
                .unwrap_or(Self::Other),
            _ => Either::Other,
        }
    }
}

impl UserData for CFrame {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "new",
            |_context,
             arguments: (
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
            )| {
                let arguments = (
                    arguments.0.map(Either::from),
                    arguments.1.map(Either::from),
                    arguments.2.map(Either::from),
                );
                match arguments {
                    (None, None, None) => Ok(Self::from_position(0.0, 0.0, 0.0)),
                    (Some(Either::Number(x)), None, None) => {
                        Ok(Self::from_position(x as f32, 0.0, 0.0))
                    }
                    (Some(Either::Number(x)), Some(Either::Number(y)), None) => {
                        Ok(Self::from_position(x as f32, y as f32, 0.0))
                    }
                    (Some(Either::Number(x)), Some(Either::Number(y)), Some(Either::Number(z))) => {
                        Ok(Self::from_position(x as f32, y as f32, z as f32))
                    }
                    (Some(Either::Vector(position)), None, None) => {
                        Ok(CFrameValue::new(DomCFrame::new(
                            position.inner(),
                            // TODO: replace with `rbx_dom_weak::types::Matrix3::identity()` once
                            // a version higher than 0.3.0 of rbx_types ships
                            Matrix3::new(
                                Vector3::new(1.0, 0.0, 0.0),
                                Vector3::new(0.0, 1.0, 0.0),
                                Vector3::new(0.0, 0.0, 1.0),
                            ),
                        )))
                    }
                    _ => Err(rlua::Error::external(
                        "invalid argument #1 to 'new' (Vector3 expected)",
                    )),
                }
            },
        );
    }
}
