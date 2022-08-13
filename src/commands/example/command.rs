use clap::{Command,arg,value_parser, ArgMatches};
use log::{info};

use super::subcommands::mysubcommand;

pub const NAME: &str = "example";

pub fn init() -> Command<'static> {
    let cmd = Command::new(NAME)
        .about("This is an example command.")
        .args(&[
            arg!(--"manifest-path" <PATH>)
                .required(false)
                .value_parser(value_parser!(std::path::PathBuf)),
        ])
        .subcommand(mysubcommand::command::init());
    
    cmd
}

pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        Some((mysubcommand::command::NAME, matches)) => mysubcommand::command::handle(matches),
        _ => {
            info!("Example command detected: {:?}", matches);
        },
    };
}