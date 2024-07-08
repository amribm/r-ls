use clap::{Arg, ArgAction, Command};
use std::{env::args_os, fs, io};

mod options {
    pub const ALL: &str = "show all the files";
    pub const LIST: &str = "show as a list";
    pub const DIRS: &str = "directory";
}

fn main() -> io::Result<()> {
    let matches = app().get_matches_from(args_os());

    let is_all = matches.get_flag(options::ALL);

    let mut dirs: Vec<String> = match matches.get_many::<String>(options::DIRS) {
        Some(s) => s.map(|x| x.to_string()).collect(),
        None => vec![String::new()],
    };

    dirs.sort();

    for dir in dirs {
        list_dir(dir)?;
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

fn list_dir(dir: String) -> io::Result<()> {
    println!("{}:", dir);
    let files = fs::read_dir(dir)?.map(|f| f.unwrap());

    for file in files {
        println!("{:>6?}", file);
    }

    Ok(())
}
