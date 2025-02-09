use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};

use clap::{Arg, Command};

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
        .after_help("Author: Isabelle <patchydev@proton.me>")
        .get_matches();

    let mut signatures: HashMap<Vec<u8>, &str> = HashMap::new();
    signatures.insert(vec![0x89, 0x50, 0x4E, 0x47], "PNG Image");

    let file_path = matches.get_one::<String>("file").unwrap();

    let mut file = File::open(file_path)?;

    let mut buffer = [0u8; 8];

    file.read_exact(&mut buffer)?;

    let file_type = signatures
        .iter()
        .find(|(sig, _)| buffer.starts_with(sig))
        .map(|(_, name)| *name)
        .unwrap_or("Unknown file type.");

    println!("Detected file type: {}", file_type);

    Ok(())
}