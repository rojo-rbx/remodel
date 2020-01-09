//! Defines how to turn RbxValue values into Lua values and back.

use rbx_dom_weak::{RbxValue, RbxValueType};
use rlua::{
    Context, MetaMethod, Result as LuaResult, ToLua, UserData, UserDataMethods, Value as LuaValue,
};
use std::fmt;

pub fn rbxvalue_to_lua<'lua>(
    context: Context<'lua>,
    value: &RbxValue,
) -> LuaResult<LuaValue<'lua>> {
    use RbxValue::*;

    fn unimplemented_type(name: &str) -> LuaResult<LuaValue<'_>> {
        Err(rlua::Error::external(format!(
            "Values of type {} are not yet implemented.",
            name
        )))
    }

    match value {
        BinaryString { value: _ } => unimplemented_type("BinaryString"),
        BrickColor { value: _ } => unimplemented_type("BrickColor"),
        Bool { value } => value.to_lua(context),
        CFrame { value: _ } => unimplemented_type("CFrame"),
        Color3 { value } => Color3Value::new(value.clone()).to_lua(context),
        Color3uint8 { value } => Color3uint8Value::new(value.clone()).to_lua(context),
        ColorSequence { value: _ } => unimplemented_type("ColorSequence"),
        Content { value } => value.as_str().to_lua(context),
        Enum { value: _ } => unimplemented_type("Enum"),
        Float32 { value } => value.to_lua(context),
        Float64 { value } => value.to_lua(context),
        Int32 { value } => value.to_lua(context),
        Int64 { value } => value.to_lua(context),
        NumberRange { value: _ } => unimplemented_type("NumberRange"),
        NumberSequence { value: _ } => unimplemented_type("NumberSequence"),
        PhysicalProperties { value: _ } => unimplemented_type("PhysicalProperties"),
        Ray { value: _ } => unimplemented_type("Ray"),
        Rect { value: _ } => unimplemented_type("Rect"),
        Ref { value: _ } => unimplemented_type("Ref"),
        SharedString { value: _ } => unimplemented_type("SharedString"),
        String { value } => value.as_str().to_lua(context),
        UDim { value: _ } => unimplemented_type("UDim"),
        UDim2 { value: _ } => unimplemented_type("UDim2"),
        Vector2 { value: _ } => unimplemented_type("Vector2"),
        Vector2int16 { value: _ } => unimplemented_type("Vector2int16"),
        Vector3 { value: _ } => unimplemented_type("Vector3"),
        Vector3int16 { value: _ } => unimplemented_type("Vector3int16"),

        _ => Err(rlua::Error::external(format!(
            "The type '{:?}' is unknown to Remodel, please file a bug!",
            value.get_type()
        ))),
    }
}

pub fn lua_to_rbxvalue(ty: RbxValueType, value: LuaValue<'_>) -> LuaResult<RbxValue> {
    match (ty, value) {
        (RbxValueType::String, LuaValue::String(lua_string)) => Ok(RbxValue::String {
            value: lua_string.to_str()?.to_owned(),
        }),
        (RbxValueType::Content, LuaValue::String(lua_string)) => Ok(RbxValue::String {
            value: lua_string.to_str()?.to_owned(),
        }),

        (RbxValueType::Bool, LuaValue::Boolean(value)) => Ok(RbxValue::Bool { value }),

        (RbxValueType::Float32, LuaValue::Number(value)) => Ok(RbxValue::Float32 {
            value: value as f32,
        }),
        (RbxValueType::Float32, LuaValue::Integer(value)) => Ok(RbxValue::Float32 {
            value: value as f32,
        }),

        (RbxValueType::Float64, LuaValue::Number(value)) => Ok(RbxValue::Float64 {
            value: value as f64,
        }),
        (RbxValueType::Float64, LuaValue::Integer(value)) => Ok(RbxValue::Float64 {
            value: value as f64,
        }),

        (RbxValueType::Int32, LuaValue::Number(value)) => Ok(RbxValue::Int32 {
            value: value as i32,
        }),
        (RbxValueType::Int32, LuaValue::Integer(value)) => Ok(RbxValue::Int32 {
            value: value as i32,
        }),

        (RbxValueType::Int64, LuaValue::Number(value)) => Ok(RbxValue::Int64 {
            value: value as i64,
        }),
        (RbxValueType::Int64, LuaValue::Integer(value)) => Ok(RbxValue::Int64 {
            value: value as i64,
        }),

        (RbxValueType::Color3, LuaValue::UserData(ref user_data)) => {
            let color = &*user_data.borrow::<Color3Value>()?;
            Ok(color.into())
        }
        (RbxValueType::Color3uint8, LuaValue::UserData(ref user_data)) => {
            let color = &*user_data.borrow::<Color3uint8Value>()?;
            Ok(color.into())
        }

        (_, unknown_value) => Err(rlua::Error::external(format!(
            "The Lua value {:?} could not be converted to the Roblox type {:?}",
            unknown_value, ty
        ))),
    }
}

pub fn type_from_str(name: &str) -> Option<RbxValueType> {
    use RbxValueType::*;

    match name {
        "BinaryString" => Some(BinaryString),
        "BrickColor" => Some(BrickColor),
        "Bool" => Some(Bool),
        "CFrame" => Some(CFrame),
        "Color3" => Some(Color3),
        "Color3uint8" => Some(Color3uint8),
        "ColorSequence" => Some(ColorSequence),
        "Content" => Some(Content),
        "Enum" => Some(Enum),
        "Float32" => Some(Float32),
        "Float64" => Some(Float64),
        "Int32" => Some(Int32),
        "Int64" => Some(Int64),
        "NumberRange" => Some(NumberRange),
        "NumberSequence" => Some(NumberSequence),
        "PhysicalProperties" => Some(PhysicalProperties),
        "Ray" => Some(Ray),
        "Rect" => Some(Rect),
        "Ref" => Some(Ref),
        "SharedString" => Some(SharedString),
        "String" => Some(String),
        "UDim" => Some(UDim),
        "UDim2" => Some(UDim2),
        "Vector2" => Some(Vector2),
        "Vector2int16" => Some(Vector2int16),
        "Vector3" => Some(Vector3),
        "Vector3int16" => Some(Vector3int16),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct Color3Value {
    value: [f32; 3],
}

impl fmt::Display for Color3Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.value[0], self.value[1], self.value[2])
    }
}

impl Color3Value {
    pub fn new(value: [f32; 3]) -> Self {
        Self { value }
    }

    fn meta_index<'lua>(
        &self,
        context: Context<'lua>,
        key: &str,
    ) -> rlua::Result<rlua::Value<'lua>> {
        match key {
            "r" => self.value[0].to_lua(context),
            "g" => self.value[1].to_lua(context),
            "b" => self.value[2].to_lua(context),
            _ => Err(rlua::Error::external(format!(
                "'{}' is not a valid member of Color3",
                key
            ))),
        }
    }
}

impl From<&Color3Value> for RbxValue {
    fn from(color: &Color3Value) -> RbxValue {
        RbxValue::Color3 { value: color.value }
    }
}

impl UserData for Color3Value {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.to_string().to_lua(context)
        });

        methods.add_meta_method(MetaMethod::Index, |context, this, key: String| {
            this.meta_index(context, &key)
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct Color3uint8Value {
    value: [u8; 3],
}

impl fmt::Display for Color3uint8Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.value[0], self.value[1], self.value[2])
    }
}

impl Color3uint8Value {
    pub fn new(value: [u8; 3]) -> Self {
        Self { value }
    }
}

impl From<&Color3uint8Value> for RbxValue {
    fn from(color: &Color3uint8Value) -> RbxValue {
        RbxValue::Color3uint8 { value: color.value }
    }
}

impl UserData for Color3uint8Value {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.to_string().to_lua(context)
        });
    }
}
