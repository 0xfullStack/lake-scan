use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    Result
};

use serde::Serialize;
use crate::models::{Pair, Protocol};

#[actix_web::get("/protocols")]
pub async fn protocols() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().body(format!("Not Implemented"))
    )
}

#[actix_web::get("/lps/{address}")]
pub async fn liquidity_pool(path: web::Path<String>) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().json(
            Pair {
                id: 0,
                pair_address: path.into_inner(),
                pair_index: 0,
                token0: "".to_string(),
                token1: "".to_string(),
                reserve0: 2,
                reserve1: 1,
                factory: "".to_string()
            }
        )
    )
}

#[actix_web::get("/pool-balance/{address}")]
pub async fn pool_balance(path: web::Path<String>) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().body(format!("Not Implemented"))
    )
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

