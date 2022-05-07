use diesel::prelude::*;
use diesel::table;
use crate::db::schema::pairs::dsl::{id as pair_id, *};
use crate::db::schema::protocols::dsl::{id as protocol_id, name as protocol_name, *};

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

pub fn get_protocols(conn: &PgConnection) -> Result<Option<Vec<Protocol>>, DBError> {
    let protocol_list = protocols
        .order(protocol_id.desc())
        .load::<Protocol>(conn)
        .optional()?;
    Ok(protocol_list)
}
// pub fn get_protocol_by_name(name: &str, conn: &PgConnection) -> Result<Option<Vec<Protocol>>, DBError> {
//     let protocol = protocols
//         .filter(protocol_name.like(name.to_string()))
//         .load::<Protocol>(conn)
//         .optional()?;
//     Ok(protocol)
// }

pub fn get_protocol_by_address(address: &str, conn: &PgConnection) -> Result<Option<Protocol>, DBError> {
    let protocol = protocols
        .filter(factory_address.eq(address))
        .first::<Protocol>(conn)
        .optional()?;
    Ok(protocol)
}

pub fn get_pairs(conn: &PgConnection) -> Result<Option<Vec<Pair>>, DBError> {
    let pair_list = pairs
        .order(pair_id.desc())
        .load::<Pair>(conn)
        .optional()?;
    Ok(pair_list)
}

type DBError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_pair_by_address(address: &str, conn: &PgConnection) -> Result<Option<Pair>, DBError> {
    let pair = pairs
        .filter(pair_address.eq(address))
        .first::<Pair>(conn)
        .optional()?;
    Ok(pair)
}
