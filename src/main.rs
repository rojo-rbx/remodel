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
pub struct Options {
    #[structopt(subcommand)]
    subcommand: Subcommand,

    /// The .ROBLOSECURITY cookie to use for authenticating to the Roblox API.
    ///
    /// Remodel will attempt to use an existing session from Roblox Studio on
    /// Windows if it is installed and you are logged in.
    ///
    /// Can also be passed via the REMODEL_AUTH environment variable.
    #[structopt(
        long = "auth",
        env = "REMODEL_AUTH",
        hide_env_values = true,
        global = true
    )]
    auth_cookie: Option<String>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    /// Run a Lua 5.3 script by path or defined in a .remodel directory.
    ///
    /// Additional arguments are passed to the script being run.
    Run {
        /// Name of .remodel script or path to a script to run.
        ///
        /// Pass `-` to read a script from stdin.
        script: String,

        /// Arguments to pass to the script as a list of strings.
        args: Vec<String>,
    },
}

fn start() -> Result<(), Box<dyn Error>> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let opt = Options::from_args();
    let auth_cookie = opt.auth_cookie.or_else(get_auth_cookie);

    match opt.subcommand {
        Subcommand::Run { script, args } => {
            let (contents, chunk_name) = load_script(&script)?;
            let lua = Lua::new();

            lua.context(move |context| {
                let lua_args = args
                    .into_iter()
                    .skip(1)
                    .map(|value| value.to_lua(context))
                    .collect::<Result<Vec<_>, _>>()?;

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
    }
}

/// Load the script from the given CLI-supplied path.
///
/// Returns the contents of the script followed by its chunk name that should be
/// given to Lua.
fn load_script(script: &str) -> io::Result<(String, String)> {
    // Passing `-` indicates that the script should be read from stdin.
    if script == "-" {
        let mut contents = String::new();
        io::stdin().read_to_string(&mut contents)?;

        return Ok((contents, "stdin".to_owned()));
    }

    log::trace!("Reading script from {}", script);

    match fs::read_to_string(script) {
        // If the input is an exact file name that exists, we'll run that
        // script.
        Ok(contents) => {
            let file_name = Path::new(script)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            Ok((contents, file_name))
        }

        Err(full_path_err) => {
            // If the given script was not a file that exists, we'll also try to
            // search for it in `.remodel/<script>.lua`.
            if full_path_err.kind() == io::ErrorKind::NotFound {
                // If the script contains path-like components, the user
                // definitely meant it as a path. To avoid path traversal
                // issues, we won't try to check `.remodel/`.
                if script.contains('/') || script.contains('\\') {
                    return Err(full_path_err);
                }

                let mut remodel_path = PathBuf::from(".remodel");
                remodel_path.push(format!("{}.lua", script));

                log::trace!("Reading script from {}", remodel_path.display());

                match fs::read_to_string(remodel_path) {
                    Ok(contents) => Ok((contents, script.to_owned())),
                    Err(remodel_err) => {
                        if remodel_err.kind() == io::ErrorKind::NotFound {
                            Err(full_path_err)
                        } else {
                            Err(remodel_err)
                        }
                    }
                }
            } else {
                Err(full_path_err)
            }
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
