#[macro_use]
extern crate diesel;

mod db;
mod handler;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest,
                body::BoxBody,
                http::header::ContentType};

use tokio::io::AsyncSeek;
use db::postgres::State;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = State::init();
    let eth = chain::ethereum::Ethereum::init();
    eth.start_sync_from(1).await;

    HttpServer::new(|| {
        App::new()
            .route("/lps", web::get().to(liquidity_pool()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
