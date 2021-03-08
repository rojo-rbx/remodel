//! Defines how to turn Variant values into Lua values and back.

use rbx_dom_weak::types::{Color3, Color3uint8, Variant, VariantType, Vector3int16};
use rlua::{
    Context, MetaMethod, Result as LuaResult, ToLua, UserData, UserDataMethods, Value as LuaValue,
};
use std::fmt;

pub fn rbxvalue_to_lua<'lua>(context: Context<'lua>, value: &Variant) -> LuaResult<LuaValue<'lua>> {
    fn unimplemented_type(name: &str) -> LuaResult<LuaValue<'_>> {
        Err(rlua::Error::external(format!(
            "Values of type {} are not yet implemented.",
            name
        )))
    }

    match value {
        Variant::BinaryString(value) => {
            base64::encode(AsRef::<[u8]>::as_ref(value)).to_lua(context)
        }
        Variant::BrickColor(_) => unimplemented_type("BrickColor"),
        Variant::Bool(value) => value.to_lua(context),
        Variant::CFrame(_) => unimplemented_type("CFrame"),
        Variant::Color3(value) => Color3Value::new(value.clone()).to_lua(context),
        Variant::Color3uint8(value) => Color3uint8Value::new(value.clone()).to_lua(context),
        Variant::ColorSequence(_) => unimplemented_type("ColorSequence"),
        Variant::Content(value) => AsRef::<str>::as_ref(value).to_lua(context),
        Variant::Enum(_) => unimplemented_type("Enum"),
        Variant::Float32(value) => value.to_lua(context),
        Variant::Float64(value) => value.to_lua(context),
        Variant::Int32(value) => value.to_lua(context),
        Variant::Int64(value) => value.to_lua(context),
        Variant::NumberRange(_) => unimplemented_type("NumberRange"),
        Variant::NumberSequence(_) => unimplemented_type("NumberSequence"),
        Variant::PhysicalProperties(_) => unimplemented_type("PhysicalProperties"),
        Variant::Ray(_) => unimplemented_type("Ray"),
        Variant::Rect(_) => unimplemented_type("Rect"),
        Variant::Ref(_) => unimplemented_type("Ref"),
        Variant::SharedString(_) => unimplemented_type("SharedString"),
        Variant::String(value) => value.as_str().to_lua(context),
        Variant::UDim(_) => unimplemented_type("UDim"),
        Variant::UDim2(_) => unimplemented_type("UDim2"),
        Variant::Vector2(_) => unimplemented_type("Vector2"),
        Variant::Vector2int16(_) => unimplemented_type("Vector2int16"),
        Variant::Vector3(_) => unimplemented_type("Vector3"),
        Variant::Vector3int16(value) => Vector3int16Value::new(value.clone()).to_lua(context),

        _ => Err(rlua::Error::external(format!(
            "The type '{:?}' is unknown to Remodel, please file a bug!",
            value.ty()
        ))),
    }
}

pub fn lua_to_rbxvalue(ty: VariantType, value: LuaValue<'_>) -> LuaResult<Variant> {
    match (ty, value) {
        (VariantType::String, LuaValue::String(lua_string)) => {
            Ok(Variant::String(lua_string.to_str()?.to_owned()))
        }
        (VariantType::Content, LuaValue::String(lua_string)) => {
            Ok(Variant::String(lua_string.to_str()?.to_owned()))
        }

        (VariantType::Bool, LuaValue::Boolean(value)) => Ok(Variant::Bool(value)),

        (VariantType::Float32, LuaValue::Number(value)) => Ok(Variant::Float32(value as f32)),
        (VariantType::Float32, LuaValue::Integer(value)) => Ok(Variant::Float32(value as f32)),

        (VariantType::Float64, LuaValue::Number(value)) => Ok(Variant::Float64(value as f64)),
        (VariantType::Float64, LuaValue::Integer(value)) => Ok(Variant::Float64(value as f64)),

        (VariantType::Int32, LuaValue::Number(value)) => Ok(Variant::Int32(value as i32)),
        (VariantType::Int32, LuaValue::Integer(value)) => Ok(Variant::Int32(value as i32)),

        (VariantType::Int64, LuaValue::Number(value)) => Ok(Variant::Int64(value as i64)),
        (VariantType::Int64, LuaValue::Integer(value)) => Ok(Variant::Int64(value as i64)),

        (VariantType::Color3, LuaValue::UserData(ref user_data)) => {
            let color = &*user_data.borrow::<Color3Value>()?;
            Ok(color.into())
        }
        (VariantType::Color3uint8, LuaValue::UserData(ref user_data)) => {
            let color = &*user_data.borrow::<Color3uint8Value>()?;
            Ok(color.into())
        }

        (VariantType::Vector3int16, LuaValue::UserData(ref user_data)) => {
            let vector3int16 = &*user_data.borrow::<Vector3int16Value>()?;
            Ok(vector3int16.into())
        }

        (VariantType::BinaryString, LuaValue::String(lua_string)) => Ok(Variant::BinaryString(
            base64::decode(lua_string)
                .map_err(rlua::Error::external)?
                .into(),
        )),

        (_, unknown_value) => Err(rlua::Error::external(format!(
            "The Lua value {:?} could not be converted to the Roblox type {:?}",
            unknown_value, ty
        ))),
    }
}

pub fn guess_type_from_rbxvalue(value: &LuaValue<'_>) -> Option<VariantType> {
    match value {
        rlua::Value::Boolean(_) => Some(VariantType::Bool),
        rlua::Value::Integer(_) | rlua::Value::Number(_) => Some(VariantType::Float64),
        rlua::Value::String(_) => Some(VariantType::String),
        rlua::Value::UserData(userdata) => {
            if userdata.is::<Vector3int16Value>() {
                Some(VariantType::Vector3)
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn type_from_str(name: &str) -> Option<VariantType> {
    use VariantType::*;

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
struct Color3Value(Color3);

impl fmt::Display for Color3Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0.r, self.0.g, self.0.b)
    }
}

impl Color3Value {
    pub fn new(value: Color3) -> Self {
        Self(value)
    }

    fn meta_index<'lua>(
        &self,
        context: Context<'lua>,
        key: &str,
    ) -> rlua::Result<rlua::Value<'lua>> {
        match key {
            "r" => self.0.r.to_lua(context),
            "g" => self.0.g.to_lua(context),
            "b" => self.0.b.to_lua(context),
            _ => Err(rlua::Error::external(format!(
                "'{}' is not a valid member of Color3",
                key
            ))),
        }
    }
}

impl From<&Color3Value> for Variant {
    fn from(color: &Color3Value) -> Variant {
        Variant::Color3(color.0)
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
struct Color3uint8Value(Color3uint8);

impl fmt::Display for Color3uint8Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0.r, self.0.g, self.0.b)
    }
}

impl Color3uint8Value {
    pub fn new(value: Color3uint8) -> Self {
        Self(value)
    }
}

impl From<&Color3uint8Value> for Variant {
    fn from(color: &Color3uint8Value) -> Variant {
        Variant::Color3uint8(color.0)
    }
}

impl UserData for Color3uint8Value {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.to_string().to_lua(context)
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3int16Value(Vector3int16);

impl fmt::Display for Vector3int16Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0.x, self.0.y, self.0.z)
    }
}

impl Vector3int16Value {
    pub fn new(value: Vector3int16) -> Self {
        Self(value)
    }
}

impl From<&Vector3int16Value> for Variant {
    fn from(color: &Vector3int16Value) -> Variant {
        Variant::Vector3int16(color.0)
    }
}

impl UserData for Vector3int16Value {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Eq, |context, this, rhs: Vector3int16Value| {
            (this.0 == rhs.0).to_lua(context)
        });

        methods.add_meta_method(MetaMethod::ToString, |context, this, _arg: ()| {
            this.to_string().to_lua(context)
        });
    }
}
