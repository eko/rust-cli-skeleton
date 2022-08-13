mod commands;
mod log;

use clap::{arg,Command, App};
use commands::{example};
use ::log::LevelFilter;
use crate::log::logger;

fn init() -> App<'static> {
    let cmd = Command::new("rust-cli-skeleton")
        .version("0.0.1")
        .bin_name("rust-cli-skeleton")
        .author("Vincent Composieux")
        .about("A simple Rust CLI skeleton")
        .arg(
            arg!(-v --verbose "Enable verbose mode").required(false),
        )
        .subcommand_required(true)
        .subcommand(example::command::init());

    cmd
}

fn main() {
    let cmd = init();
    let matches = cmd.get_matches();

    let logger_level = if matches.is_present("verbose") {
        LevelFilter::Trace
    } else {
        LevelFilter::Error
    };

    logger::init(logger_level);

    match matches.subcommand() {
        Some((example::command::NAME, matches)) => example::command::handle(matches),
        _ => unreachable!("Unknown command."),
    };
}
