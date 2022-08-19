-- Your SQL goes here

-- chains --
CREATE TABLE IF NOT EXISTS chains (
    id SERIAL PRIMARY KEY,
    chain_id VARCHAR(32) UNIQUE NOT NULL,
    chain_name VARCHAR(128) NOT NULL,
    icon_url VARCHAR(256) NULL,
    website VARCHAR(256) NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);

-- blocks --
CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    chain_id INT NOT NULL,
    height BIGINT NOT NULL,
    block_hash VARCHAR(256) NOT NULL,
    prev_hash VARCHAR(256) NOT NULL,
    proposer_address VARCHAR(256) NOT NULL,
    last_commit_hash VARCHAR(256) NOT NULL,
    data_hash VARCHAR(256) NOT NULL,
    validators_hash VARCHAR(256) NOT NULL,
    next_validators_hash VARCHAR(256) NOT NULL,
    consensus_hash VARCHAR(256) NOT NULL,
    app_hash VARCHAR(256) NOT NULL,
    last_result_hash VARCHAR(256) NOT NULL,
    evidence_hash VARCHAR(256) NOT NULL,
    block_time TIMESTAMP NOT NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);
CREATE INDEX idx_blocks_height ON blocks(height);
CREATE INDEX idx_blocks_block_hash ON blocks(block_hash);

-- transactions --
CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    chain_id INT NOT NULL,
    transaction_hash VARCHAR(256) UNIQUE NOT NULL,
    height BIGINT NOT NULL,
    code int NOT NULL,
    code_space VARCHAR(256) NOT NULL,
    tx_data TEXT NOT NULL,
    raw_log TEXT NOT NULL,
    info TEXT NOT NULL,
    memo VARCHAR(1024),
    gas_wanted BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    tx_timestamp VARCHAR(256) NOT NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);
CREATE INDEX idx_transactions_tx_hash ON transactions(transaction_hash);

-- events --
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    chain_id INT NOT NULL,
    tx_type SMALLINT NOT NULL, -- 0: transaction, 1: begin_block, 2: after_block
    tx_hash VARCHAR(256), -- it can be null
    event_type VARCHAR(256) NOT NULL,
    event_key VARCHAR(256) NOT NULL,
    event_value VARCHAR(256) NOT NULL,
    indexed BOOLEAN NOT NULL,
    inserted_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);
