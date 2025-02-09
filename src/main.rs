mod utils;

use std::{
    fs::File,
    io::{Read, Result},
    process,
};

use utils::{args, filesystem, signatures};

fn main() -> Result<()> {
    let matches = args::setup_args();

    let file_path = matches.get_one::<String>("file").unwrap();
    let verbose = matches.get_one::<bool>("verbose").unwrap();

    if filesystem::validate_file(file_path).is_err() {
        println!("{}", filesystem::validate_file(file_path).unwrap_err());
        process::exit(1);
    }

    if *verbose {
        println!("{} is a valid file. Loading list of signatures...", file_path);
    }

    let sigs = signatures::load_signatures();

    let mut file = File::open(file_path)?;

    let mut buffer = [0u8; 10];

    if *verbose {
        println!("Reading first 10 bytes of file...");
    }

    file.read_exact(&mut buffer)?;

    if *verbose {
        let hex_string = buffer.iter().map(|b| format!("0x{:02X}", b)).collect::<Vec<String>>().join(" ");
        println!("First 10 bytes: {}", hex_string);
    }

    let file_type = signatures::check_signature(&buffer, &sigs);

    println!("Detected file type: {}", file_type.unwrap_or("Unknown file type"));

    Ok(())
}