

use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    body::BoxBody
};

use serde::Serialize;
use crate::db::models::{Pair, Protocol};

#[get("/lps/{address}")]
async fn liquidity_pool(address: web::Path<String>) -> HttpResponse {

}

async fn index() -> HttpResponse {
    HttpResponse::Ok().json("Hello")
}

impl Responder for Pair {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
