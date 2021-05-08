use rlua::{UserData, UserDataMethods, Value as LuaValue};

use crate::value::{CFrameValue, Vector3Value};

pub struct CFrame;

impl CFrame {
    fn from_position(x: f32, y: f32, z: f32) -> CFrameValue {
        CFrameValue::new(rbx_dom_weak::types::CFrame::new(
            rbx_dom_weak::types::Vector3::new(x as f32, y as f32, z as f32),
            // TODO: replace with `rbx_dom_weak::types::Matrix3::identity()` once
            // a version higher than 0.3.0 of rbx_types ships
            rbx_dom_weak::types::Matrix3::new(
                rbx_dom_weak::types::Vector3::new(1.0, 0.0, 0.0),
                rbx_dom_weak::types::Vector3::new(0.0, 1.0, 0.0),
                rbx_dom_weak::types::Vector3::new(0.0, 0.0, 1.0),
            ),
        ))
    }
}

impl UserData for CFrame {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function(
            "new",
            |_context,
             triplet: (
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
                Option<LuaValue<'_>>,
            )| {
                match triplet {
                    (None, None, None) => Ok(Self::from_position(0.0, 0.0, 0.0)),
                    (Some(LuaValue::Number(x)), None, None) => {
                        Ok(Self::from_position(x as f32, 0.0, 0.0))
                    }
                    (Some(LuaValue::Integer(x)), None, None) => {
                        Ok(Self::from_position(x as f32, 0.0, 0.0))
                    }
                    (Some(LuaValue::Number(x)), Some(LuaValue::Number(y)), None) => {
                        Ok(Self::from_position(x as f32, y as f32, 0.0))
                    }
                    (Some(LuaValue::Number(x)), Some(LuaValue::Integer(y)), None) => {
                        Ok(Self::from_position(x as f32, y as f32, 0.0))
                    }
                    (Some(LuaValue::Integer(x)), Some(LuaValue::Integer(y)), None) => {
                        Ok(Self::from_position(x as f32, y as f32, 0.0))
                    }
                    (Some(LuaValue::Integer(x)), Some(LuaValue::Number(y)), None) => {
                        Ok(Self::from_position(x as f32, y as f32, 0.0))
                    }
                    (
                        Some(LuaValue::Number(x)),
                        Some(LuaValue::Number(y)),
                        Some(LuaValue::Number(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Number(x)),
                        Some(LuaValue::Integer(y)),
                        Some(LuaValue::Number(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Number(x)),
                        Some(LuaValue::Number(y)),
                        Some(LuaValue::Integer(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Number(x)),
                        Some(LuaValue::Integer(y)),
                        Some(LuaValue::Integer(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Integer(x)),
                        Some(LuaValue::Integer(y)),
                        Some(LuaValue::Integer(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Integer(x)),
                        Some(LuaValue::Number(y)),
                        Some(LuaValue::Integer(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Integer(x)),
                        Some(LuaValue::Integer(y)),
                        Some(LuaValue::Number(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (
                        Some(LuaValue::Integer(x)),
                        Some(LuaValue::Number(y)),
                        Some(LuaValue::Number(z)),
                    ) => Ok(Self::from_position(x as f32, y as f32, z as f32)),
                    (Some(LuaValue::UserData(user_data)), None, None) => {
                        let position = &*user_data.borrow::<Vector3Value>()?;
                        Ok(CFrameValue::new(rbx_dom_weak::types::CFrame::new(
                            (*position).into(),
                            // TODO: replace with `rbx_dom_weak::types::Matrix3::identity()` once
                            // a version higher than 0.3.0 of rbx_types ships
                            rbx_dom_weak::types::Matrix3::new(
                                rbx_dom_weak::types::Vector3::new(1.0, 0.0, 0.0),
                                rbx_dom_weak::types::Vector3::new(0.0, 1.0, 0.0),
                                rbx_dom_weak::types::Vector3::new(0.0, 0.0, 1.0),
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
