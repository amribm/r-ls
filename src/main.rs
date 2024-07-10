use clap::{Arg, ArgAction, Command};
use std::ffi::OsString;
use std::path::Path;
use std::{env::args_os, fs, io};
use thiserror::Error;

mod options {
    pub const ALL: &str = "show all the files";
    pub const LIST: &str = "show as a list";
    pub const DIRS: &str = "directory";
}

#[derive(Debug, Error)]
enum LsError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("unable to unwrap error")]
    DirUnwrapErr,
}

fn main() -> io::Result<()> {
    let matches = app().get_matches_from(args_os());

    let is_all = matches.get_flag(options::ALL);

    let mut dirs = match matches.get_many::<OsString>(options::DIRS) {
        Some(s) => s.map(|x| Path::new(x)).collect(),
        None => vec![Path::new(".")],
    };

    dirs.sort();

    for dir in dirs {
        list_dir(dir, RatConfig::new())?;
    }

    Ok(())
}

fn app() -> Command {
    Command::new("r-ls")
        .about("simple `ls` written in rust")
        .arg(
            Arg::new(options::ALL)
                .short('a')
                .action(ArgAction::SetTrue)
                .overrides_with(options::ALL),
        )
        .arg(Arg::new(options::DIRS).action(ArgAction::Append))
    // .arg(
    //     Arg::new(options::LIST)
    //         .short('l')
    //         .action(ArgAction::SetTrue)
    //         .overrides_with(options::LIST),
    // )
}

struct RatConfig {
    all: bool,
}

impl RatConfig {
    fn new() -> RatConfig {
        RatConfig { all: true }
    }
}

fn list_dir(dir: &Path, opts: RatConfig) -> Result<(), LsError> {
    println!("{:?}:", dir.file_name());
    let files = fs::read_dir(dir)?;

    for file in files {}

    Ok(())
}
