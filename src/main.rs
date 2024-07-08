use std::{fs, io};

fn main() -> io::Result<()> {
    let files = fs::read_dir(".")?;

    for file in files {
        let file_name = file?;
        println!("{:?}", file_name.file_name())
    }

    Ok(())
}
