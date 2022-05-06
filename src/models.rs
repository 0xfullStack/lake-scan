
use diesel::prelude::*;
use diesel::prelude::{Queryable, QueryResult};
use serde_derive::{Serialize, Deserialize};
use diesel::table;

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