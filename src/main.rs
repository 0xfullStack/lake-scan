// use actix_web::{get, web, App, HttpServer, Responder};
//
//
// #[actix_web::main] // or #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(greet)
//     })
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
//
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }
//

use diesel::prelude::*;
use lake_scan::{
    *,
    models::Protocol,
    schema::protocols::dsl::*
};

fn main() {

    let connection = &mut establish_connection();



}
