


use std::env;

use dotenv;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use super::schema::protocols;
use super::schema::pairs;

/*
    let _protocol = create_protocol(
        &connection,
        "Uniswap Protocol",
        Some("https://uniswap.org/"),
        "ETHEREUM_MAINNET",
        Some("Swap, earn, and build on the leading decentralized crypto trading protocol."),
        Some("uniswap-v2"),
        "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
        "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"
    );
 */
pub fn create_protocol<'a>(
    conn: &PgConnection,
    name: &'a str,
    official_url: Option<&'a str>,
    network: &'a str,
    description: Option<&'a str>,
    symbol: Option<&'a str>,
    router_address: &'a str,
    factory_address: &'a str) -> Protocol {

    let new_protocol = NewProtocol { name, official_url, network, description, symbol, router_address, factory_address };
    diesel::insert_into(protocols::table)
        .values(&new_protocol)
        .get_result(conn)
        .expect("Error saving new protocol")
}

pub fn delete_protocol<'a>(conn: &PgConnection, name: &'a str) {

    let num_deleted = diesel::delete(protocols::table.filter( protocols::dsl::name.like(name)))
        .execute(conn)
        .expect("Error deleting protocol");
}

pub fn select_protocols<'a>(conn: &PgConnection, factory_address: &'a str) {
    let results = protocols::table
        .load::<Protocol>(conn)
        .expect("Error loading protocols");

    for protocol in results {
        println!("{}", protocol.name);
        println!("-----------");

        match protocol.description {
            None => {}
            Some(value) => {
                println!("{}", value);
            }
        }
    }
}

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

