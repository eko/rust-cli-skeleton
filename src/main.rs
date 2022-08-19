mod commands;
mod log;
mod grpc;

use clap::{arg,Command, App};
use commands::{hello, grpc as command_grpc};
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
        .subcommand(command_grpc::server::init())
        .subcommand(hello::command::init());

    cmd
}

fn main() {
    let cmd = init();
    let matches = cmd.get_matches();

    let logger_level = if matches.is_present("verbose") {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    logger::init(logger_level);

    match matches.subcommand() {
        Some((hello::command::NAME, matches)) => hello::command::handle(matches),
        Some((command_grpc::server::NAME, matches)) => command_grpc::server::handle(matches),
        _ => unreachable!("Unknown command."),
    };
}
