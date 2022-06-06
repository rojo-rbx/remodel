use mlua::{FromLua, Lua, Table, ToLua, UserData, UserDataMethods, Value as LuaValue};
use serde::Serialize;
use serde_json::{
    ser::{PrettyFormatter, Serializer},
    Number, Value as JsonValue,
};

pub struct Json;

impl UserData for Json {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("toString", |_context, lua_value: Value| {
            serde_json::to_string(&lua_value.0).map_err(mlua::Error::external)
        });

        methods.add_function(
            "toStringPretty",
            |_context, (lua_value, indent): (Value, Option<String>)| {
                let pretty_formatter = if let Some(ref indent) = indent {
                    PrettyFormatter::with_indent(indent.as_bytes())
                } else {
                    PrettyFormatter::new()
                };

                let mut output = Vec::new();
                let mut serializer = Serializer::with_formatter(&mut output, pretty_formatter);
                lua_value
                    .0
                    .serialize(&mut serializer)
                    .map_err(mlua::Error::external)?;

                String::from_utf8(output).map_err(mlua::Error::external)
            },
        );

        methods.add_function("fromString", |_context, source: String| {
            serde_json::from_str::<JsonValue>(&source)
                .map(Value)
                .map_err(mlua::Error::external)
        });
    }
}

pub struct Value(pub JsonValue);

impl<'lua> ToLua<'lua> for Value {
    fn to_lua(self, context: &'lua Lua) -> mlua::Result<LuaValue<'lua>> {
        match self.0 {
            JsonValue::Null => Ok(LuaValue::Nil),
            JsonValue::Bool(value) => Ok(LuaValue::Boolean(value)),
            JsonValue::Number(num) => {
                if let Some(value) = num.as_i64() {
                    Ok(LuaValue::Integer(value))
                } else if let Some(value) = num.as_f64() {
                    Ok(LuaValue::Number(value))
                } else {
                    Err(mlua::Error::external(
                        "Numbers should be representable by either i64 or f64",
                    ))
                }
            }
            JsonValue::String(value) => value.to_lua(context),
            JsonValue::Array(values) => {
                let table = context.create_table()?;

                for (i, value) in values.into_iter().enumerate() {
                    table.raw_set(i + 1, Value(value))?;
                }

                Ok(LuaValue::Table(table))
            }
            JsonValue::Object(values) => {
                let table = context.create_table()?;

                for (key, value) in values {
                    table.raw_set(key, Value(value))?;
                }

                Ok(LuaValue::Table(table))
            }
        }
    }
}

impl<'lua> FromLua<'lua> for Value {
    fn from_lua(lua_value: LuaValue<'lua>, _context: &'lua Lua) -> mlua::Result<Self> {
        lua_to_json(lua_value).map(Value)
    }
}

fn lua_to_json<'lua>(lua_value: LuaValue<'lua>) -> mlua::Result<JsonValue> {
    match lua_value {
        LuaValue::Nil => Ok(JsonValue::Null),
        LuaValue::Boolean(value) => Ok(JsonValue::Bool(value)),

        // TODO: Better way to preserve integer accuracy?
        LuaValue::Integer(value) => Ok(JsonValue::Number(Number::from_f64(value as f64).unwrap())),
        LuaValue::Number(value) => Ok(JsonValue::Number(Number::from_f64(value).unwrap())),

        LuaValue::String(lua_str) => lua_str
            .to_str()
            .map(|value| JsonValue::String(value.to_owned())),

        LuaValue::Table(table) => match classify_table(table.clone())? {
            TableKind::Sparse(capacity) => {
                let mut map = serde_json::Map::with_capacity(capacity);

                for pair in table.pairs::<String, Value>() {
                    let (key, value) = pair?;

                    map.insert(key, value.0);
                }

                Ok(JsonValue::Object(map))
            }
            TableKind::ArrayLike(capacity) => {
                let mut array = vec![JsonValue::Null; capacity];

                for pair in table.pairs::<usize, Value>() {
                    let (key, value) = pair?;

                    array[key - 1] = value.0;
                }

                Ok(JsonValue::Array(array))
            }
        },

        _ => Err(mlua::Error::external("Value cannot be turned into JSON")),
    }
}

enum TableKind {
    Sparse(usize),
    ArrayLike(usize),
}

fn classify_table<'lua>(table: Table<'lua>) -> mlua::Result<TableKind> {
    let mut highest_key = 0;
    let mut total_keys = 0;
    let mut has_non_whole_keys = false;

    for pair in table.pairs::<LuaValue, LuaValue>() {
        total_keys += 1;

        if !has_non_whole_keys {
            let (key, _value) = pair?;

            match key {
                LuaValue::Integer(value) if value > 0 => {
                    let value = value as usize;
                    highest_key = highest_key.max(value);
                }
                LuaValue::Number(value) if value.fract() == 0.0 && value > 0.0 => {
                    let value = value as usize;
                    highest_key = highest_key.max(value);
                }

                // This table can't be represented as an array!
                _ => has_non_whole_keys = true,
            }
        }
    }

    if has_non_whole_keys {
        return Ok(TableKind::Sparse(total_keys));
    }

    // A perfect, non-sparse array!
    if highest_key == total_keys {
        return Ok(TableKind::ArrayLike(highest_key));
    }

    // For tables that are at least 50% resident and have entirely positive
    // numeric keys, serialize them as an array.
    let density = (total_keys as f32) / (highest_key as f32);
    if density > 0.5 {
        Ok(TableKind::ArrayLike(highest_key))
    } else {
        Ok(TableKind::Sparse(total_keys))
    }
}
