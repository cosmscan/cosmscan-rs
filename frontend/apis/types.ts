export let BASE_URL = "http://localhost:1337";

export interface Chain {
    id: number;
    chain_id: string;
    chain_name: string;
    icon_url: string | null;
    website: string | null;
}

export interface Block {
    id: number;
    chain_id: number;
    height: number;
    block_hash: string;
    prev_hash: string;
    proposer_address: string
    last_commit_hash: string;
    data_hash: string;
    validator_hash: string;
    next_validator_hash: string;
    consensus_hash: string;
    app_hash: string;
    last_results_hash: string;
    evidence_hash: string;
    block_time: string;
    inserted_at: string;
}

export interface ListBlock {
    total: number;
    blocks: Block[];
}

export interface Transaction {
    id: number;
    chain_id: number;
    transaction_hash: string;
    height: number;
    code: number;
    code_space: string;
    tx_data: string;
    raw_log: string;
    info: string;
    memo: string;
    gas_wanted: number;
    gas_used: number;
    tx_timestamp: string;
    inserted_at: string;
}

export interface ListTransaction {
    total: number;
    transactions: Transaction[];
}

export interface Event {
    tx_type: number;
    tx_hash: string;
    event_seq: number;
    event_type: string;
    event_key: string;
    event_value: string;
    indexed: boolean;
}

export interface MessageType {
    type: string;
}

export interface Message extends MessageType {
    [key: string]: any;
}

export interface TransactionDetail extends Transaction {
    events: Event[];
    messages: Message[];
}