mod json;
mod remodel;

use rlua::Context;

pub use json::Json;
pub use remodel::Remodel;

pub struct RemodelApi;

impl RemodelApi {
    pub fn inject(context: Context<'_>) -> rlua::Result<()> {
        context.globals().set("remodel", Remodel)?;
        context.globals().set("json", Json)?;

        Ok(())
    }
}
