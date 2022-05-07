#[macro_use]
extern crate diesel;

mod db;
mod dex;
mod lend;
mod nft;

use actix_web::{App, HttpServer, dev, guard, http, HttpRequest, HttpResponse, Responder, Result, web};
use actix_web::body::EitherBody;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::{
    DefaultHeaders, ErrorHandlerResponse, ErrorHandlers, Logger
};
use std::env;
use env_logger;
use env_logger::Env;
use db::postgres::*;
use dotenv::dotenv;

use crate::{ dex::handlers as dex_handlers };

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // DB pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url).expect("Failed to create pool");

    HttpServer::new( move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().add(("version", "0.1")))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, server_error))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/dex")
                    .guard(guard::Header("content-type", "application/json"))
                    .service(dex_handlers::liquidity_pool)
                    .service(dex_handlers::protocols)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn server_error<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();
    let res = res.set_body(r#"{"json", "in the body"}"#.to_owned());

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}


