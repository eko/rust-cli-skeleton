use clap::{Command, ArgMatches, arg};
use log::{info};

use crate::grpc::server;

pub const NAME: &str = "grpc:server";

pub fn init() -> Command<'static> {
    let cmd = Command::new(NAME)
        .about("Runs the gRPC server.")
        .args(&[
            arg!(--"addr" "0.0.0.0:8000")
                .required(false),
        ]);
    
    cmd
}

pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        _ => {
            let addr: String = match matches.value_of("addr") {
                Some(addr) => addr.to_string(),
                None => "0.0.0.0:8000".to_string(),
            };

            info!("Running gRPC server on port {}", addr);
            server::start(addr).unwrap();
        },
    };
}