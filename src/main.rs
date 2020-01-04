mod auth_cookie;
mod remodel_api;
mod remodel_context;
mod roblox_api;
mod value;

#[cfg(feature = "unstable_rojo_api")]
mod rojo_api;

use std::{
    error::Error,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use rlua::{Lua, MultiValue, ToLua};
use structopt::StructOpt;

use crate::{
    auth_cookie::get_auth_cookie, remodel_api::RemodelApi, remodel_context::RemodelContext,
    roblox_api::RobloxApi,
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
    #[structopt(long = "auth", env = "REMODEL_AUTH", hide_env_values = true)]
    auth_cookie: Option<String>,
}

fn start() -> Result<(), Box<dyn Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let opt = Options::from_args();
    let (contents, chunk_name) = load_script(&opt.script)?;

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

        #[cfg(feature = "unstable_rojo_api")]
        {
            rojo_api::RojoApi::inject(context)?;
        }

        let chunk = context.load(&contents).set_name(&chunk_name)?;
        chunk.call(MultiValue::from_vec(lua_args))
    })?;

    Ok(())
}

/// Load the script from the given CLI-supplied path.
///
/// Returns the contents of the script followed by its chunk name that should be
/// given to Lua.
fn load_script(script: &Path) -> io::Result<(String, String)> {
    // Passing `-` indicates that the script should be read from stdin.
    if script.as_os_str() == "-" {
        let mut contents = String::new();
        io::stdin().read_to_string(&mut contents)?;

        return Ok((contents, "stdin".to_owned()));
    }

    log::trace!("Reading script from {}", script.display());

    match fs::read_to_string(script) {
        // If the input is an exact file name that exists, we'll run that
        // script.
        Ok(contents) => {
            let file_name = script.file_name().unwrap().to_string_lossy().into_owned();

            Ok((contents, file_name))
        }

        Err(err) => {
            // If we couldn't find the exact name we're looking for, but a
            // script with that name inside a folder named `.remodel/` exists,
            // we'll run that!
            if script.is_relative() {
                if let Some(command_name) = script.file_name().and_then(|name| name.to_str()) {
                    let mut command_path = PathBuf::from(".remodel");
                    command_path.push(format!("{}.lua", command_name));

                    log::trace!("Reading script from {}", command_path.display());

                    match fs::read_to_string(command_path) {
                        Ok(contents) => {
                            return Ok((contents, command_name.to_owned()));
                        }

                        // If we got any IO errors that weren't 'file not
                        // found', we'll report those. Otherwise, reporting an
                        // error with this file path might confuse the user.
                        Err(err) => {
                            if err.kind() != io::ErrorKind::NotFound {
                                return Err(err);
                            }
                        }
                    }
                }
            }

            Err(err)
        }
    }
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
