use std::future::Future;
use std::ops::Deref;
use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    Result,
    Error
};
use diesel::QueryResult;

use serde::Serialize;
use crate::db::postgres::{Database, PgPool};
use crate::dex::models::*;
use super::models::{Pair, Protocol};

#[actix_web::get("/protocols")]
pub async fn protocols() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().json(
            vec![
                Pair {
                    id: 0,
                    pair_address: "test 1".to_string(),
                    token0: "".to_string(),
                    token1: "".to_string(),
                    block_number: 0,
                    block_hash: "".to_string(),
                    transaction_hash: "".to_string(),
                    reserve0: 2.to_string(),
                    reserve1: 1.to_string(),
                    factory_address: "".to_string()
                },
                Pair {
                    id: 0,
                    pair_address: "test 2".to_string(),
                    token0: "".to_string(),
                    token1: "".to_string(),
                    block_number: 0,
                    block_hash: "".to_string(),
                    transaction_hash: "".to_string(),
                    reserve0: 2.to_string(),
                    reserve1: 1.to_string(),
                    factory_address: "".to_string()
                }
            ]
        )
    )
}

#[actix_web::get("/lps/{address}")]
pub async fn liquidity_pool(pool: web::Data<PgPool>, path: web::Path<String>) -> Result<HttpResponse, Error> {
    let address = path.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    let pair = web::block(move ||
        get_pair_by_address(&address, &conn)
    )
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(pair) = pair {
        Ok(HttpResponse::Ok().json(pair))
    } else {
        // let address = path.into_inner();
        let res = HttpResponse::NotFound().body(format!("No pair found with address"));
        Ok(res)
    }
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