use clap::{Arg, ArgAction, ArgMatches, Command};

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
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show verbose output")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .after_help("Credits: Isabelle <patchydev@proton.me>")
        .get_matches()
}