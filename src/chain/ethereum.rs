
use std::env;
use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;

use web3::{
    contract::{Contract, Options},
    futures::StreamExt,
    types::{FilterBuilder, Address, H160, U256, CallRequest},
    helpers as w3h,
    ethabi::Uint,
    transports::{Http, WebSocket},
    Web3
};

use dotenv::dotenv;
use hex_literal::hex;
use web3::ethabi::ParamType::String;
use crate::db::models::Pair;
use crate::db::postgres::NewPair;

pub struct Ethereum {

}

impl Ethereum  {

    pub fn init() -> Ethereum {
        let eth = Ethereum{};
        eth
    }

    async fn check_start_index() {

    }

    // async fn get_account_balance(&self, address: &str) -> U256 {
    //     let web3 = Web3::new(self.websocket);
    //     let account = Address::from_str(address).unwrap();
    //     let balance = web3.eth().balance(account, None).await.unwrap();
    //     balance
    // }

    pub async fn start_sync_from(&self, index: i64) {

        dotenv::dotenv().ok();
        // let node_ws = &env::var("INFURA_MAINNET_WS").unwrap();
        let node_http = &env::var("INFURA_MAINNET_HTTP").unwrap();
        let http = Http::new(node_http).unwrap();

        let web3 = Web3::new(http);
        let factory_address = Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap();
        let factory_contract = Contract::from_json(web3.eth(), factory_address, include_bytes!("../abi/uniswap_v2_factory.json")).unwrap();

        let pairs_length: Uint = factory_contract
            .query("allPairsLength", (), None, Options::default(), None)
            .await
            .unwrap();
        let pair_address: Address = factory_contract
            .query("allPairs", (42_u32,), None, Options::default(), None)
            .await
            .unwrap();
        let pair_contract = Contract::from_json(web3.eth(), pair_address, include_bytes!("../abi/uniswap_v2_pair.json")).unwrap();
        let pair_token0: Address = pair_contract.query("token0", (), None, Options::default(), None).await.unwrap();
        let pair_token1: Address = pair_contract.query("token1", (), None, Options::default(), None).await.unwrap();
        // let token_reserves = pair_contract.query("getReserves", (), None, Options::default(), None).await.unwrap();


        let new_pair = NewPair {
            pair_address: w3h::to_string(&pair_address),
            pair_index: index,
            token0: w3h::to_string(&pair_token0),
            token1: w3h::to_string(&pair_token1),
            reserve0: 1,
            reserve1: 1,
            factory: w3h::to_string(&factory_address),
            created_at_timestamp: Option::None,
            created_at_block_number: Option::None
        };

        println!("got new_pair: {:?}", new_pair);
    }

    async fn handle_pair_created_event(&self) {

        // let web3 = Web3::new(self.websocket);
        // let uniswap_factory_addr = Address::from_str(&env::var("INFURA_MAINNET").unwrap()).unwrap();
        //
        // let uniswap_factory_contract = Contract::from_json(web3.eth(), uniswap_factory_addr, include_bytes!("../abi/uniswap_v2_factory.json")).unwrap();
        //
        //
        // // Filter for Hello event in our contract
        // let filter = FilterBuilder::default()
        //     .address(vec![uniswap_factory_contract.address()])
        //     .topics(
        //         Some(vec![hex!("0d3648bd0f6ba80134a33ba9275ac585d9d315f0ad8355cddefde31afa28d0e9").into()]),
        //         None,
        //         None,
        //         None,
        //     ).build();
        //
        // let filter = web3.eth_filter().create_logs_filter(filter).await?;
        //
        // let logs_stream = filter.stream(Duration::from_secs(1));
        //
        // web3::futures::pin_mut!(logs_stream);
        //
        // let log = logs_stream.next().await.unwrap();
        // println!("got log: {:?}", log);
    }

    async fn handle_liquidity_added_event() {}
    async fn handle_liquidity_removed_event() {}
}