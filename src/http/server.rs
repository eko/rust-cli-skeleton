use actix_web::{App, HttpServer, web};
use super::handler::{hello};

#[actix_web::main]
pub async fn start(addr: String) -> std::io::Result<()> {
    let host = addr.split(":").next().unwrap();
    let port = addr.split(":").skip(1).next().unwrap().parse::<u16>().unwrap();

    HttpServer::new(|| {
        App::new()
        .route("/hello/{name}", web::get().to(hello::handler))
    })
    .bind((host, port))?
    .run()
    .await
}