use actix_web::{
    web, HttpRequest, HttpResponse, Responder,
    body::BoxBody,
    Result
};

use serde::Serialize;

#[actix_web::get("/balance/{address}")]
pub async fn balance(path: web::Path<String>) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok().body(format!("Not implemented: {}", path.into_inner()))
    )
}
