-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_blocks_height;
DROP INDEX IF EXISTS idx_blocks_block_hash;
DROP INDEX IF EXISTS idx_transactions_tx_hash;

DROP TABLE IF EXISTS chains;
DROP TABLE IF EXISTS blocks;
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS events;