use std::ops::Deref;
use actix::{Handler, Message};
use actix_web::{Error, error};
use crate::db::postgres::Database;
use super::models::{ Pair, Protocol };

pub struct GetPairs;
pub struct GetProtocols;

impl Message for GetPairs {
    type Result = Result<Vec<Pair>, Error>;
}
impl Message for GetProtocols {
    type Result = Result<Vec<Protocol>, Error>;
}

impl Handler<GetPairs> for Database {

    type Result = Result<Vec<Pair>, Error>;

    fn handle(&mut self, msg: GetPairs, ctx: &mut Self::Context) -> Self::Result {
        Pair::get_pairs(self.get_connection()?.deref())
            .map_err(|_| error::ErrorInternalServerError("Failed to retrive pairs"))
    }
}

impl Handler<GetProtocols> for Database {

    type Result = Result<Vec<Protocol>, Error>;

    fn handle(&mut self, msg: GetProtocols, ctx: &mut Self::Context) -> Self::Result {
        Protocol::get_protocols(self.get_connection()?.deref())
            .map_err(|_| error::ErrorInternalServerError("Failed to retrive pairs"))
    }
}
