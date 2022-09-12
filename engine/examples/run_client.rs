use std::{env::args, sync::Arc};

use cosmoscout_engine::{
    client::{Client, ClientConfig},
    errors::Error,
    utils::bytes_to_tx_hash,
};

use futures::future;
use log::info;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let block_height = args().nth(1).unwrap().parse::<u64>().unwrap();

    let tendermint = "http://localhost:26657";
    let grpc = "http://localhost:9090";
    let rest = "http://localhost:1317";

    let client = Client::new(ClientConfig {
        tendermint_rpc_endpoint: tendermint.to_string(),
        grpc_endpoint: grpc.to_string(),
        rest_api_endpoint: rest.to_string(),
    })
    .await
    .map(Mutex::new)
    .map(Arc::new)?;

    let block = client.clone().lock().await.get_block(block_height).await?;
    let block_result = client
        .clone()
        .lock()
        .await
        .get_block_result(block_height)
        .await?;
    let txes = block.data.iter().map(|d| bytes_to_tx_hash(d)).map(|hash| {
        let client = client.clone();
        async move { client.lock().await.get_transaction(hash).await }
    });

    info!(
        "block height: {}, hash: {}",
        block_height,
        block.header.hash()
    );

    let txes_infos = future::join_all(txes).await;
    for tx in txes_infos.iter() {
        match tx {
            Ok(tx) => {
                let body = tx.tx_response.as_ref().unwrap();
                info!("tx hash: {}", body.txhash);
                info!("tx events: {:?}", body.events);
            }
            Err(e) => {
                info!("tx error: {}", e);
            }
        }
    }

    let raw_txes = txes_infos
        .iter()
        .map(|tx| tx.as_ref().unwrap())
        .collect::<Vec<_>>();
    let raw_events = client
        .clone()
        .lock()
        .await
        .extract_events(raw_txes, &block_result);

    info!("transaction events");
    for event in raw_events.iter() {
        info!("event: {:?}", event);
    }

    info!("transaction messages");
    for tx in txes_infos.iter() {
        match tx {
            Ok(tx) => {
                let body = tx.tx_response.as_ref().unwrap();
                let messages = client
                    .clone()
                    .lock()
                    .await
                    .get_tx_messages(body.txhash.clone())
                    .await
                    .unwrap();
                info!("tx hash: {}", body.txhash);
                info!("tx messages: {:?}", messages);
            }
            Err(e) => {
                info!("tx error: {}", e);
            }
        }
    }

    Ok(())
}
