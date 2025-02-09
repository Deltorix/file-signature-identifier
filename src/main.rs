use std::fs::File;
use std::io::{self, Read};

use clap::{Arg, Command};

fn main() -> io::Result<()> {
    let matches = Command::new("File Signature Identifier")
        .version("0.0.1")
        .author("Isabelle <patchydev@proton.me")
        .about("Quickly identify file type")
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .help("The file to identify type of")
                .required(true),
        )
        .after_help("Author: Isabelle <patchydev@proton.me>")
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();

    let mut file = File::open(file_path)?;

    let mut buffer = [0u8; 8];

    let bytes = file.read(&mut buffer)?;

    if bytes > 0 {
        for byte in &buffer[..bytes] {
            print!("{:02x}", byte);
        }

        println!();
    } else {
        println!("No bytes read");
    }

    Ok(())
}