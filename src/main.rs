mod utils;

use std::fs::File;
use std::io::{Read, Result};
use std::process;
use clap::{Arg, Command};

use utils::signatures;
use utils::filesystem;


fn main() -> Result<()> {
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
        .after_help("Credits: Isabelle <patchydev@proton.me>")
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();

    if filesystem::validate_file(file_path).is_err() {
        println!("{}", filesystem::validate_file(file_path).unwrap_err());
        process::exit(1);
    }

    let sigs = signatures::load_signatures();

    let mut file = File::open(file_path)?;

    let mut buffer = [0u8; 10];

    file.read_exact(&mut buffer)?;

    let file_type = signatures::check_signature(&buffer, &sigs);

    println!("Detected file type: {}", file_type.unwrap_or("Unknown file type"));

    Ok(())
}