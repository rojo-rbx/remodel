use std::{error::Error, fs, path::PathBuf};

use rlua::Lua;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(parse(from_os_str))]
    script: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Options::from_args();
    let contents = fs::read_to_string(&opt.script)?;
    let lua = Lua::new();

    lua.context(|context| {
        let chunk = context.load(&contents);
        chunk.exec()
    })?;

    Ok(())
}
