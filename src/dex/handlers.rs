use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    Result,
    Error
};
use crate::db::postgres::PgPool;
use crate::dex::models::{get_latest_pair_reserves, get_pair_by_address, Pair, Protocol, PairRes};

#[actix_web::get("/protocols")]
pub async fn protocols() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().json(
            Vec::<Pair>::new()
        )
    )
}

#[actix_web::get("/lps/{address}")]
pub async fn liquidity_pool(pool: web::Data<PgPool>, path: web::Path<String>) -> Result<HttpResponse, Error> {
    let address = path.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let clone_address = address.clone();
    let clone_conn = pool.get().expect("couldn't get db connection from pool");

    let pair = web::block(move ||
        get_pair_by_address(&address, &conn)
    )
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let reserves = web::block(move ||
        get_latest_pair_reserves(&clone_address, &clone_conn)
    )
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(pair) = pair {
        let pair_res = PairRes {
            id: pair.id,
            pair_address: pair.pair_address,
            factory_address: pair.factory_address,
            token0: pair.token0,
            token1: pair.token1,
            block_number: pair.block_number,
            block_hash: pair.block_hash,
            transaction_hash: pair.transaction_hash,
            reserve0: reserves.0,
            reserve1: reserves.1,
        };
        Ok(HttpResponse::Ok().json(pair_res))
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