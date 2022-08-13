use clap::{ArgMatches,Command};
use log::{info};

pub const NAME: &str = "mysubcommand";

pub fn init() -> Command<'static> {
    let cmd = Command::new(NAME)
        .about("This is an example subcommand called mysubcommand.")
    ;

    cmd
}

pub fn handle(matches: &ArgMatches) {
    info!("Example mysubcommand detected: {:?}", matches);
}