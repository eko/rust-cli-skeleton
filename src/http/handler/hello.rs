use actix_web::{web, Responder};

pub async fn handler(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}