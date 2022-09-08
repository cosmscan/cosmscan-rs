-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_blocks_chain_id;
DROP INDEX IF EXISTS idx_blocks_height;
DROP INDEX IF EXISTS idx_blocks_block_hash;
DROP INDEX IF EXISTS idx_accounts_chain_id;
DROP INDEX IF EXISTS idx_events_chain_id;
DROP INDEX IF EXISTS idx_events_tx_hash;
DROP INDEX IF EXISTS idx_transactions_chain_id;
DROP INDEX IF EXISTS idx_transactions_tx_hash;

DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS account_balance;
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS blocks;
DROP TABLE IF EXISTS chains;