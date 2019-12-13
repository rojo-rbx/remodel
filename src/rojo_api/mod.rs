//! Bindings to Rojo's API, exposed via Remodel.

mod project;

use rlua::{Context, UserData, UserDataMethods};

pub struct RojoApi;

impl RojoApi {
    pub fn inject(context: Context<'_>) -> rlua::Result<()> {
        context.globals().set("Rojo", Rojo)?;

        Ok(())
    }
}

struct Rojo;

impl UserData for Rojo {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("hello", |_context, _args: ()| {
            println!("Hello, from Rojo!");
            Ok(())
        });
    }
}
