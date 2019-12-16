mod auth_cookie;
mod remodel_api;
mod remodel_context;
mod roblox_api;
mod rojo_api;
mod value;

use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};

use rlua::{Lua, MultiValue, ToLua};
use structopt::StructOpt;

use crate::{
    auth_cookie::get_auth_cookie, remodel_api::RemodelApi, remodel_context::RemodelContext,
    roblox_api::RobloxApi, rojo_api::RojoApi,
};

#[derive(Debug, StructOpt)]
#[structopt(
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Options {
    /// The Lua 5.3 script to run. Pass `-` to read from stdin.
    #[structopt(parse(from_os_str))]
    script: PathBuf,

    /// Arguments to pass to the script as a list of strings.
    script_arguments: Vec<String>,

    /// The .ROBLOSECURITY cookie to use for authenticating to the Roblox API.
    #[structopt(long = "auth")]
    auth_cookie: Option<String>,
}

fn start() -> Result<(), Box<dyn Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let opt = Options::from_args();

    let (contents, chunk_name) = if opt.script.as_os_str() == "-" {
        let mut contents = String::new();
        io::stdin().read_to_string(&mut contents)?;

        (contents, "stdin".to_owned())
    } else {
        let contents = fs::read_to_string(&opt.script)?;
        let file_name = opt
            .script
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        (contents, file_name)
    };

    let lua = Lua::new();

    lua.context(move |context| {
        let lua_args = opt
            .script_arguments
            .into_iter()
            .map(|value| value.to_lua(context))
            .collect::<Result<Vec<_>, _>>()?;

        let auth_cookie = opt.auth_cookie.or_else(get_auth_cookie);

        RemodelContext::new(auth_cookie).inject(context)?;

        RemodelApi::inject(context)?;
        RobloxApi::inject(context)?;
        RojoApi::inject(context)?;

        let chunk = context.load(&contents).set_name(&chunk_name)?;
        chunk.call(MultiValue::from_vec(lua_args))
    })?;

    Ok(())
}

fn main() {
    match start() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
