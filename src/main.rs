mod auth_cookie;
mod remodel_api;
mod remodel_context;
mod roblox_api;

use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::PathBuf,
};

use rlua::{Lua, MultiValue, ToLua};
use structopt::StructOpt;

use crate::{remodel_api::RemodelApi, remodel_context::RemodelContext, roblox_api::RobloxApi};

#[derive(Debug, StructOpt)]
#[structopt(
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Options {
    /// The input script to run. Should be valid Lua 5.3. Pass `-` to read from
    /// stdin.
    #[structopt(parse(from_os_str))]
    script: PathBuf,

    /// Arguments to pass to the script as a list of strings.
    script_arguments: Vec<String>,
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

        RemodelContext::inject(context)?;

        RemodelApi::inject(context)?;
        RobloxApi::inject(context)?;

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
