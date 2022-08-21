use clap::{Command, ArgMatches, arg};
use log::{info};

use crate::http::server;

pub const NAME: &str = "http:server";

pub fn init() -> Command<'static> {
    let cmd = Command::new(NAME)
        .about("Runs the HTTP server.")
        .args(&[
            arg!(--"addr" "0.0.0.0:8080")
                .required(false),
        ]);
    
    cmd
}

pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        _ => {
            let addr: String = match matches.value_of("addr") {
                Some(addr) => addr.to_string(),
                None => "0.0.0.0:8080".to_string(),
            };

            info!("Running HTTP server on port {}", addr);
            server::start(addr).unwrap();
        },
    };
}