#[macro_use]
extern crate diesel;

mod db;
mod handlers;
mod models;

use actix_web::{dev, guard, http, HttpRequest, HttpResponse, Responder, Result, web};
use actix_web::body::EitherBody;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::{
    DefaultHeaders, ErrorHandlerResponse, ErrorHandlers, Logger
};

use handlers::{dex, lend, nft};
use env_logger;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().add(("version", "0.1")))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, server_error))
            .service(
                web::scope("/dex")
                    .guard(guard::Header("content-type", "application/json"))
                    .service(dex::liquidity_pool)
                    .service(dex::pool_balance)
            )
            .service(
                web::scope("/lend")
                    .service(lend::balance)
            )
            // .service(default::handler::index)
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

pub mod default {
    pub mod handler {
        use actix_web::HttpResponse;
        pub async fn index() -> HttpResponse {
            HttpResponse::Ok()
                .content_type("plain/text")
                .append_header(("rusher", "rushing"))
                .body("rusher")
        }
    }
}


