table! {
    Pair (id) {
        id -> Int8,
        pair_address -> Bpchar,
        factory_address -> Bpchar,
        token0 -> Bpchar,
        token1 -> Bpchar,
        block_number -> Int8,
        block_hash -> Text,
        transaction_hash -> Text,
        reserve0 -> Text,
        reserve1 -> Text,
    }
}

table! {
    Protocol (id) {
        id -> Int8,
        name -> Varchar,
        official_url -> Nullable<Varchar>,
        network -> Varchar,
        description -> Nullable<Text>,
        symbol -> Nullable<Varchar>,
        router_address -> Bpchar,
        factory_address -> Bpchar,
    }
}

allow_tables_to_appear_in_same_query!(
    Pair,
    Protocol
);
