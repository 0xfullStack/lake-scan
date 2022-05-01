
use std::env;
use std::ops::Add;
use std::str::FromStr;

use web3::ethabi::Uint;
use web3::types::{H160, U256, CallRequest};
use web3::helpers as w3h;

use hex_literal::hex;
use std::time;
use web3::{
    contract::{Contract, Options},
    futures::StreamExt,
    types::{FilterBuilder, Address}
};

use crate::db::postgres::NewPair;
use crate::db::schema::pairs::{factory, pair_address, token0};

// async fn fetch_all_pairs(factory: &str) -> Result<Uint, E> {
//     Result::Ok(Uint::from(1))
// }
//
// async fn fetch_pair_address(index: Uint) -> Result<String, E> {
//     Result::Ok(String::from("dddd"))
// }

// async fn assemble_pair_info(factory: Address, pair_address: Address) -> Result<NewPair, E> {
//
//     dotenv::dotenv().ok();
//     let node = &env::var("INFURA_MAINNET").unwrap().as_str();
//     let http = web3::transports::Http::new(node);
//     let web3s = web3::Web3::new(http);
//
//     let address = Address::from_str("0x3139Ffc91B99aa94DA8A2dc13f1fC36F9BDc98eE").unwrap();
//     let contract = Contract::from_json(web3s.eth(), address, include_bytes!("uniswap_v2_pair.json")).unwrap();
//
//     let token0: Address = contract.query("token0", (), None, Options::default(), None).await.unwrap();
//     let token1: Address = contract.query("token1", (), None, Options::default(), None).await.unwrap();
//     let reserves: Address = contract.query("getReserves", (), None, Options::default(), None).await.unwrap();
//
//     println!("got token0: {:?}", token0);
//
//     Result::Ok(NewPair{
//         pair_address: pair_address::to_string().as_str(),
//         pair_index: 0,
//         token0: token0::to_string().as_str(),
//         token1: token1::to_string().as_str(),
//         reserve0: 0,
//         reserve1: 0,
//         factory: factory::to_string().as_str(),
//         created_at_timestamp: None,
//         created_at_block_number: None
//     })
// }

// async fn main() -> web3::contract::Result<()> {
//
//     dotenv::dotenv().ok();
//     let websocket = web3::transports::WebSocket::new(&env::var("INFURA_MAINNET").unwrap()).await?;
//     let web3s = web3::Web3::new(websocket);
//
//     let uniswap_factory_addr = Address::from_str(&env::var("INFURA_MAINNET").unwrap()).unwrap();
//
//     let uniswap_factory_contract = Contract::from_json(web3s.eth(), uniswap_factory_addr, include_bytes!("../abi/uniswap_v2_factory.json")).unwrap();
//
//
//     // Filter for Hello event in our contract
//     let filter = FilterBuilder::default()
//         .address(vec![uniswap_factory_contract.address()])
//         .topics(
//             Some(vec![hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").into()]),
//             None,
//             None,
//             None,
//         )
//         .build();
//
//     let filter = web3s.eth_filter().create_logs_filter(filter).await?;
//
//
//     let logs_stream = filter.stream(time::Duration::from_secs(1));
//
//
//
//     web3::futures::pin_mut!(logs_stream);
//
//     // let tx = contract.call("hello", (), accounts[0], Options::default()).await?;
//     // println!("got tx: {:?}", tx);
//
//     let log = logs_stream.next().await.unwrap();
//     println!("got log: {:?}", log);
//
//     Ok(())
//
//     dotenv::dotenv().ok();
//     let node = &env::var("INFURA_MAINNET").unwrap().as_str();
//     let http = web3::transports::Http::new(node);
//     let web3s = web3::Web3::new(http);
//
//     let address = Address::from_str("0x3139Ffc91B99aa94DA8A2dc13f1fC36F9BDc98eE").unwrap();
//     let contract = Contract::from_json(web3s.eth(), address, include_bytes!("uniswap_v2_pair.json")).unwrap();
//
//     let token0: Address = contract.query("token0", (), None, Options::default(), None).await.unwrap();
//     let token1: Address = contract.query("token1", (), None, Options::default(), None).await.unwrap();
//     let reserves: Address = contract.query("getReserves", (), None, Options::default(), None).await.unwrap();
//
//     println!("got token0: {:?}", token0);
// }

// fn wei_to_eth(wei_val: U256) -> f64 {
//     let res = wei_val.as_u128() as f64;
//     res / 1_000_000_000_000_000_000.0
// }

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_MAINNET").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    // for account in accounts {
    //     let balance = web3s.eth().balance(account, None).await?;
    //     println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    // }

    // let uniswap_factory_addr = Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap();
    //
    // let uniswap_factory_contract = Contract::from_json(web3s.eth(), uniswap_factory_addr, include_bytes!("../abi/uniswap_v2_factory.json")).unwrap();

    // let all_pairs_length: Uint = uniswap_factory_contract
    //     .query("allPairsLength", (), None, Options::default(), None)
    //     .await
    //     .unwrap();
    //
    // let pair_address: Address = uniswap_factory_contract
    //     .query("allPairs", (42_u32,), None, Options::default(), None)
    //     .await
    //     .unwrap();

    //
    //
    // println!("All pairs length: {}, pair index: {}", all_pairs_length, w3h::to_string(&pair_address));


    let address = Address::from_str("0x3139Ffc91B99aa94DA8A2dc13f1fC36F9BDc98eE").unwrap();

    let contract = Contract::from_json(web3s.eth(), address, include_bytes!("../abi/uniswap_v2_pair.json")).unwrap();
    // let contract = Contract::from_json(web3s.eth(), address, include_bytes!("uniswap_v2_pair.json")).unwrap();

    let token_0: Address = contract.query("token0", (), None, Options::default(), None).await.unwrap();
    // let token1: Address = contract.query("token1", (), None, Options::default(), None).await.unwrap();
    // let reserves: Address = contract.query("getReserves", (), None, Options::default(), None).await.unwrap();

    println!("got token0: {}", w3h::to_string(&token_0));

    Ok(())
}




//
// extern crate ethabi;
// extern crate tokio_core;
// extern crate web3;
// use std::str::FromStr;
// use web3::types::{BlockNumber, FilterBuilder, H256};
//
// fn main() {
//     let infura_http = "https://kovan.infura.io/v3/6fdc99560fce488cba4a52b6c8c0574b";
//     let mut eloop = tokio_core::reactor::Core::new().unwrap();
//     let web3 = web3::Web3::new(
//         web3::transports::Http::with_event_loop(infura_http, &eloop.handle(), 1).unwrap(),
//     );
//     let sig =
//         H256::from_str("f11a7558a113d9627989c5edf26cbd19143b7375248e621c8e30ac9e0847dc3f").unwrap();
//     println!("{:?}", sig);
//     let filter = FilterBuilder::default()
//         .topics(Some(vec![sig]), None, None, None)
//         .from_block(BlockNumber::Earliest)
//         .build();
//
//     let event_future = web3.eth().logs(filter);
//     let result = eloop.run(event_future);
//     println!("result {:?}", result);
// }