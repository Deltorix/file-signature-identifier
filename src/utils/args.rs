use clap::{Arg, ArgMatches, Command};

pub fn setup_args() -> ArgMatches {
    Command::new("File Signature Identifier")
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
        .get_matches()
}