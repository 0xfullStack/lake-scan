pub mod schema;
pub mod postgres;
pub mod assembler;

use diesel::pg::PgConnection;
use diesel::prelude::QueryResult;
use diesel::prelude::*;
use super::db::schema::{pairs, protocols};

use super::models::{ Protocol, Pair };

impl Protocol {
    pub fn get_protocols(conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
        protocols::dsl::protocols.order(protocols::id.desc()).load::<Protocol>(conn)
    }
    // pub fn get_protocol_by_name(name: &str, conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
    //     pairs::dsl::pairs.filter(protocols::name.like(name.to_string())).load::<Protocol>(conn)
    // }
    // pub fn get_protocol_by_address(address: &str, conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
    //     pairs::dsl::pairs.filter(protocols::factory_address.eq_all(address)).load::<Protocol>(conn)
    // }
}

impl Pair {
    pub fn get_pairs(conn: &PgConnection) -> QueryResult<Vec<Pair>> {
        pairs::dsl::pairs.order(pairs::id.desc()).load::<Pair>(conn)
    }
    pub fn get_pair_by(address: &str, conn: &PgConnection) -> QueryResult<Vec<Pair>> {
        pairs::dsl::pairs.filter(pairs::pair_address.eq_all(address)).load::<Pair>(conn)
    }
}
