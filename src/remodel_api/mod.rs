mod json;
mod remodel;

use mlua::Lua;

pub use json::Json;
pub use remodel::Remodel;

pub struct RemodelApi;

impl RemodelApi {
    pub fn inject(context: &Lua) -> mlua::Result<()> {
        context.globals().set("remodel", Remodel)?;
        context.globals().set("json", Json)?;

        Ok(())
    }
}
