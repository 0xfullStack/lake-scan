use diesel::prelude::*;
use diesel::table;
use crate::db::schema::{pairs, protocols};

use serde_derive::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Protocol {
    pub id: i64,
    pub name: String,
    pub official_url: Option<String>,
    pub network: String,
    pub description: Option<String>,
    pub symbol: Option<String>,
    pub router_address: String,
    pub factory_address: String
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Pair {
    pub id: i64,
    pub pair_address: String,
    pub pair_index: i64,
    pub token0: String,
    pub token1: String,
    pub reserve0: i64,
    pub reserve1: i64,
    pub factory: String
}

impl Protocol {
    pub fn get_protocols(conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
        protocols::dsl::protocols.order(protocols::id.desc()).load::<Protocol>(conn)
    }
    pub fn get_protocol_by_name(name: &str, conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
        protocols::dsl::protocols.filter(protocols::name.like(name.to_string())).load::<Protocol>(conn)
    }
    pub fn get_protocol_by_address(address: &str, conn: &PgConnection) -> QueryResult<Vec<Protocol>> {
        protocols::dsl::protocols.filter(protocols::factory_address.eq_all(address)).load::<Protocol>(conn)
    }
}

impl Pair {
    pub fn get_pairs(conn: &PgConnection) -> QueryResult<Vec<Pair>> {
        pairs::dsl::pairs.order(pairs::id.desc()).load::<Pair>(conn)
    }
    pub fn get_pair_by(address: &str, conn: &PgConnection) -> QueryResult<Vec<Pair>> {
        pairs::dsl::pairs.filter(pairs::pair_address.eq_all(address)).load::<Pair>(conn)
    }
}
