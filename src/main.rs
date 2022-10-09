mod remodel_api;
mod remodel_context;
mod roblox_api;
mod sniff_type;
mod value;

use std::{
    env, fs,
    io::{self, Read},
    panic,
    path::{Path, PathBuf},
    process,
};

use backtrace::Backtrace;
use mlua::{Lua, MultiValue, ToLua};
use structopt::StructOpt;

use crate::{remodel_api::RemodelApi, remodel_context::RemodelContext, roblox_api::RobloxApi};

#[derive(Debug, StructOpt)]
#[structopt(
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Options {
    #[structopt(subcommand)]
    subcommand: Subcommand,

    /// Enables more verbose logging.
    ///
    /// Can be specified up to 3 times to increase verbosity.
    #[structopt(long("verbose"), short, global(true), parse(from_occurrences))]
    verbosity: u8,

    /// The .ROBLOSECURITY cookie to use for authenticating to the Roblox API.
    ///
    /// Remodel will attempt to use an existing session from Roblox Studio on
    /// Windows if it is installed and you are logged in.
    ///
    /// Can also be passed via the REMODEL_AUTH environment variable.
    #[structopt(long("auth"), env("REMODEL_AUTH"), hide_env_values(true), global(true))]
    auth_cookie: Option<String>,

    /// The Roblox Cloud API key to use
    #[structopt(
        long("api-key"),
        env("REMODEL_API_KEY"),
        hide_env_values(true),
        global(true)
    )]
    api_key: Option<String>,
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

fn main() {
    let options = Options::from_args();
    initialize_logger(options.verbosity);
    install_panic_hook();

    if let Err(err) = run(options) {
        log::error!("{:?}", err);
        process::exit(1);
    }
}

fn run(options: Options) -> Result<(), anyhow::Error> {
    let api_key = options.api_key;
    let auth_cookie = options.auth_cookie.or_else(rbx_cookie::get_value);

    match options.subcommand {
        Subcommand::Run { script, args } => {
            let (contents, chunk_name) = load_script(&script)?;
            let lua = Lua::new();

            let lua_args = args
                .into_iter()
                .map(|value| value.to_lua(&lua))
                .collect::<Result<Vec<_>, _>>()?;

            RemodelContext::new(auth_cookie, api_key).inject(&lua)?;

            RemodelApi::inject(&lua)?;
            RobloxApi::inject(&lua)?;

            let chunk = lua.load(&contents).set_name(&chunk_name)?;
            chunk.call(MultiValue::from_vec(lua_args))?;

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

    let file_path = Path::new(script);

    match fs::read_to_string(file_path) {
        // If the input is an exact file name that exists, we'll run that
        // script.
        Ok(contents) => {
            let file_name = file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            Ok((contents, file_name))
        }

        Err(full_path_err) => {
            // If the given script was not a file that exists, or if it was a directory,
            // we'll also try to search for it in `.remodel/<script>.lua`.
            if full_path_err.kind() == io::ErrorKind::NotFound || file_path.is_dir() {
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

fn initialize_logger(verbosity: u8) {
    let log_filter = match verbosity {
        0 => "info",
        1 => "info,remodel=debug",
        2 => "info,remodel=trace",
        _ => "trace",
    };

    let log_env = env_logger::Env::default().default_filter_or(log_filter);

    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        // Indent following lines equal to the log level label, like `[ERROR] `
        .format_indent(Some(8))
        .init();
}

fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        // PanicInfo's payload is usually a &'static str or String.
        // See: https://doc.rust-lang.org/beta/std/panic/struct.PanicInfo.html#method.payload
        let message = match panic_info.payload().downcast_ref::<&str>() {
            Some(message) => message.to_string(),
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(message) => message.clone(),
                None => "<no message>".to_string(),
            },
        };

        log::error!("Remodel crashed!");
        log::error!("This may be a Remodel bug.");
        log::error!("");
        log::error!(
            "Please consider filing an issue: {}/issues",
            env!("CARGO_PKG_REPOSITORY")
        );
        log::error!("");
        log::error!("Details: {}", message);

        if let Some(location) = panic_info.location() {
            log::error!("in file {} on line {}", location.file(), location.line());
        }

        // When using the backtrace crate, we need to check the RUST_BACKTRACE
        // environment variable ourselves. Once we switch to the (currently
        // unstable) std::backtrace module, we won't need to do this anymore.
        let should_backtrace = env::var("RUST_BACKTRACE")
            .map(|var| var == "1")
            .unwrap_or(false);

        if should_backtrace {
            eprintln!("{:?}", Backtrace::new());
        } else {
            eprintln!(
                "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace."
            );
        }

        process::exit(1);
    }));
}
