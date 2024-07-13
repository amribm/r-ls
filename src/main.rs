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

    #[error("unable to unwrap directory")]
    DirUnwrapErr,
}

fn main() -> Result<(), LsError> {
    let matches = app().get_matches_from(args_os());
    let mut conf = LsConfig::new();

    conf.all = matches.get_flag(options::ALL);

    let mut dirs = matches
        .get_many::<String>(options::DIRS)
        .map(|v| v.map(Path::new).collect())
        .unwrap_or_else(|| vec![Path::new(".")]);

    if dirs.len() > 1 {
        conf.print_dir = true
    }

    dirs.sort();

    for dir in dirs {
        list_dir(dir, &conf)?;
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

struct LsConfig {
    all: bool,
    print_dir: bool,
}

impl LsConfig {
    fn new() -> LsConfig {
        LsConfig {
            all: false,
            print_dir: false,
        }
    }
}

fn list_dir(dir: &Path, opts: &LsConfig) -> Result<(), LsError> {
    if opts.print_dir {
        println!("{}:", dir.to_string_lossy().to_string());
    }
    let files = fs::read_dir(dir)?;

    for file in files {
        if let Ok(fl) = file {
            let file_name = fl.file_name().to_string_lossy().to_string();
            if !opts.all && file_name.starts_with(".") {
                continue;
            }
            println!("{file_name}")
        } else {
            return Err(LsError::DirUnwrapErr);
        };
    }

    Ok(())
}
