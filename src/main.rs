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