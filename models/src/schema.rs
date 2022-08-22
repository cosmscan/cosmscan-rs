table! {
    blocks (id) {
        id -> Int4,
        chain_id -> Int4,
        height -> Int8,
        block_hash -> Varchar,
        prev_hash -> Varchar,
        proposer_address -> Varchar,
        last_commit_hash -> Varchar,
        data_hash -> Varchar,
        validators_hash -> Varchar,
        next_validators_hash -> Varchar,
        consensus_hash -> Varchar,
        app_hash -> Varchar,
        last_result_hash -> Varchar,
        evidence_hash -> Varchar,
        block_time -> Timestamp,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    chains (id) {
        id -> Int4,
        chain_id -> Varchar,
        chain_name -> Varchar,
        icon_url -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    events (id) {
        id -> Int4,
        chain_id -> Int4,
        tx_type -> Int2,
        tx_hash -> Nullable<Varchar>,
        event_type -> Varchar,
        event_key -> Varchar,
        event_value -> Varchar,
        indexed -> Bool,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        chain_id -> Int4,
        transaction_hash -> Varchar,
        height -> Int8,
        code -> Int4,
        code_space -> Varchar,
        tx_data -> Text,
        raw_log -> Text,
        info -> Text,
        memo -> Nullable<Varchar>,
        gas_wanted -> Int8,
        gas_used -> Int8,
        tx_timestamp -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    blocks,
    chains,
    events,
    transactions,
);
