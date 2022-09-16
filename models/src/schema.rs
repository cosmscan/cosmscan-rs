// @generated automatically by Diesel CLI.

diesel::table! {
    account_balance (id) {
        id -> Int4,
        account_id -> Int4,
        amount -> Int8,
        denom -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    accounts (id) {
        id -> Int4,
        chain_id -> Int4,
        address -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
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

diesel::table! {
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

diesel::table! {
    events (id) {
        id -> Int4,
        chain_id -> Int4,
        tx_type -> Int2,
        tx_hash -> Nullable<Varchar>,
        block_height -> Int8,
        event_seq -> Int4,
        event_type -> Varchar,
        event_key -> Varchar,
        event_value -> Varchar,
        indexed -> Bool,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        transaction_id -> Int4,
        seq -> Int4,
        rawdata -> Jsonb,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
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

diesel::joinable!(account_balance -> accounts (account_id));
diesel::joinable!(messages -> transactions (transaction_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_balance,
    accounts,
    blocks,
    chains,
    events,
    messages,
    transactions,
);
