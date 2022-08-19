use tonic::{transport::Server, Request, Response, Status};
use example::demo_server::{DemoServer, Demo};
use example::{HelloResponse};

pub mod example {
    tonic::include_proto!("example");
}

#[derive(Default)]
pub struct DemoService {}

#[tonic::async_trait]
impl Demo for DemoService {
    async fn hello(&self, _: Request<()>) -> Result<Response<HelloResponse>, Status> {
        let message = HelloResponse {
            name: "world".to_string(),
        };

        Ok(Response::new(message))
    }
}

#[tokio::main]
pub async fn start(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse().unwrap();
    let demo_service = DemoService::default();

    Server::builder()
        .add_service(DemoServer::new(demo_service))
        .serve(addr)
        .await?;

    Ok(())
}
