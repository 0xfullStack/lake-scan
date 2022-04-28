#[macro_use]
extern crate diesel;

mod chain;
mod schema;
mod postgres;

use actix_web::{App, get, HttpServer, Responder, web};


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(greet) })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/lps/{address}")]
async fn liquidity_pool(address: web::Path<String>) -> impl Responder {
    format!("Hello {address}!")
}

