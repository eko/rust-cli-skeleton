use clap::{Command,arg, ArgMatches};
use log::{info};
use example::demo_client::DemoClient;

pub mod example {
    tonic::include_proto!("example");
}

pub const NAME: &str = "hello";

pub fn init() -> Command<'static> {
    let cmd = Command::new(NAME)
        .about("This is an example command.")
        .args(&[
            arg!(--"addr" "http://0.0.0.0:8000")
                .required(false),
        ]);
    
    cmd
}

pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        _ => {
            info!("Hello command detected: {:?}", matches);

            let addr: String = match matches.value_of("addr") {
                Some(addr) => addr.to_string(),
                None => "http://0.0.0.0:8000".to_string(),
            };

            send_hello(addr).unwrap();
        },
    };
}

#[tokio::main]
async fn send_hello(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DemoClient::connect(addr).await?;

    let request = tonic::Request::new(());
    let response = client.hello(request).await?;

    info!("Response received name from gRPC: {:?}", response.into_inner().name);

    Ok(())
}