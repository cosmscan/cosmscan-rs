use response::{Event, EventType};
use sha2::{Digest, Sha256};
use tendermint::abci;

pub mod client;
pub mod errors;
pub mod response;

fn convert_block_events(
    abci_events: Vec<abci::Event>,
    height: i64,
    event_type: EventType,
) -> Vec<Event> {
    abci_events
        .iter()
        .enumerate()
        .map(|(seq, evt)| {
            let event_type = event_type.clone();
            evt.attributes.iter().map(move |attr| {
                Event {
                    tx_type: event_type.clone(),
                    tx_hash: None,
                    block_height: height,
                    event_seq: seq as i32,
                    event_type: evt.type_str.clone(),
                    event_key: attr.key.to_string(),
                    event_value: attr.value.to_string(),
                    indexed: false,
                }
            })
        })
        .flatten()
        .collect::<Vec<Event>>()
}

fn bytes_to_tx_hash(data: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let tx_hash = hasher.finalize();
    format!("{:X}", tx_hash)
}
