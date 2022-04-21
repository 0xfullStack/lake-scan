use diesel::Queryable;

#[derive(Queryable)]
pub struct Protocol {
    pub id: i64,
    pub name: String,
    pub official_url: Option<String>,
    pub network: String,
    pub description: Option<String>,
    pub symbol: Option<String>,
    pub router_address: String,
    pub factory_address: String,
}

#[derive(Queryable)]
pub struct Pair {
    pub id: i64,
    pub pair_address: String,
    pub pair_index: i64,
    pub token0: String,
    pub token1: String,
    pub reserve0: String,
    pub reserve1: String,
    pub factory: String,
    pub created_at_timestamp: Option<i64>,
    pub created_at_block_number: Option<i64>,
}

use diesel::Insertable;
use super::schema::protocols;
use super::schema::pairs;

#[derive(Insertable)]
#[table_name="protocols"]
pub struct NewProtocol<'a> {
    pub name: &'a str,
    pub official_url: Option<&'a str>,
    pub network: &'a str,
    pub description: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub router_address: &'a str,
    pub factory_address: &'a str,
}

#[derive(Insertable)]
#[table_name="pairs"]
pub struct NewPair<'a> {
    pub pair_address: &'a str,
    pub pair_index: i64,
    pub token0: &'a str,
    pub token1: &'a str,
    pub reserve0: i64,
    pub reserve1: i64,
    pub factory: &'a str,
    pub created_at_timestamp: Option<i64>,
    pub created_at_block_number: Option<i64>,
}
