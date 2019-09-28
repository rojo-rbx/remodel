//! Defines how to turn RbxValue values into Lua values and back.

use rbx_dom_weak::{RbxValue, RbxValueType};
use rlua::{Context, Result as LuaResult, ToLua, Value as LuaValue};

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
        Color3 { value: _ } => unimplemented_type("Color3"),
        Color3uint8 { value: _ } => unimplemented_type("Color3uint8"),
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
