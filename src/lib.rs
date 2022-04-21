
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use models::{Protocol, NewProtocol};
use schema::protocols as ProtocolsTable;
use ProtocolsTable::dsl as ProtocolsTableDSL;

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
    diesel::insert_into(ProtocolsTable::table)
        .values(&new_protocol)
        .get_result(conn)
        .expect("Error saving new protocol")
}

pub fn delete_protocol<'a>(conn: &PgConnection, name: &'a str) {

    let num_deleted = diesel::delete(ProtocolsTable::table.filter( ProtocolsTableDSL::name.like(name)))
        .execute(conn)
        .expect("Error deleting protocol");
}

pub fn select_protocols<'a>(conn: &PgConnection, factory_address: &'a str) {
    let results = ProtocolsTable::table
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

