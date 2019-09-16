use rlua::Context;

pub struct RobloxApi;

impl RobloxApi {
    pub fn inject<'lua>(_context: Context<'lua>) -> rlua::Result<()> {
        Ok(())
    }
}
