use diesel::prelude::*;
use crate::db::schema::Pair::dsl::{*, pair_address };
use crate::db::schema::Protocol::dsl::{  *, id as protocol_id, factory_address };

use serde_derive::{Serialize, Deserialize};
use crate::db::schema::ReserveLog;
use crate::db::schema::ReserveLog::{block_number as reserveLog_block_number, log_index, pair_address as reserveLog_pair_address, reserve0, reserve1};

#[derive(Queryable, Debug, Serialize, Deserialize)]
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

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Pair {
    pub id: i64,
    pub pair_address: String,
    pub factory_address: String,
    pub token0: String,
    pub token1: String,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: String
}

pub fn get_protocols(conn: &PgConnection) -> Result<Option<Vec<Protocol>>, DBError> {
    let protocol_list = Protocol
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
    let protocol = Protocol
        .filter(factory_address.eq(address))
        .first::<Protocol>(conn)
        .optional()?;
    Ok(protocol)
}

// pub fn get_pairs(conn: &PgConnection) -> Result<Option<Vec<Pair>>, DBError> {
//     let pair_list = Pair
//         .order(pair_id.desc())
//         .load::<Pair>(conn)
//         .optional()?;
//     Ok(pair_list)
// }

type DBError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_pair_by_address(address: &str, conn: &PgConnection) -> Result<Option<Pair>, DBError> {
    let pair = Pair
        .filter(pair_address.eq(address))
        .first::<Pair>(conn)
        .optional()?;
    Ok(pair)
}

pub fn get_latest_pair_reserves(pair_address_: &str, conn: &PgConnection) -> QueryResult<(String, String)> {
    // SELECT * FROM "ReserveLog" WHERE pair_address = '0xe0cc5afc0ff2c76183416fb8d1a29f6799fb2cdf' ORDER BY (block_number, log_index) DESC
    ReserveLog::table
        .select((reserve0, reserve1))
        .filter(reserveLog_pair_address.eq(pair_address_))
        .order_by((reserveLog_block_number.desc(), log_index.desc()))
        .limit(1)
        .get_result(conn)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PairRes {
    pub id: i64,
    pub pair_address: String,
    pub factory_address: String,
    pub token0: String,
    pub token1: String,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: String,
    pub reserve0: String,
    pub reserve1: String,
}