use std::{error::Error, fs, path::PathBuf};

use rlua::Lua;
use structopt::StructOpt;

mod lua_api;

use lua_api::Remodel;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    script: PathBuf,
}

fn start() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();
    let contents = fs::read_to_string(&opt.script)?;
    let lua = Lua::new();

    lua.context(move |context| {
        context.globals().set("remodel", Remodel)?;

        let chunk = context.load(&contents);
        chunk.exec()
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
