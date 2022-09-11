use std::str::from_utf8;

use cosmos_sdk_proto::cosmos::tx::v1beta1::{service_client, GetTxResponse};
use cosmoscout_models::models::event::{
    TX_TYPE_BEGIN_BLOCK, TX_TYPE_END_BLOCK, TX_TYPE_TRANSACTION,
};
use tendermint::{abci, block};
use tendermint_rpc::{endpoint::block_results, Client as tm_client};

use crate::{errors::Error, rawdata::RawEvent};

pub struct ClientConfig {
    pub tendermint_rpc_endpoint: String,
    pub grpc_endpoint: String,
    pub rest_api_endpoint: String,
}

pub struct Client {
    tm_client: tendermint_rpc::HttpClient,
    grpc_client: service_client::ServiceClient<tonic::transport::Channel>,
    config: ClientConfig,
}

impl Client {
    /// Create a new client.
    /// It abstracts the tendermint and grpc client both.
    pub async fn new(config: ClientConfig) -> Result<Self, Error> {
        let tm_client =
            tendermint_rpc::HttpClient::new(config.tendermint_rpc_endpoint.clone().as_str())
                .map_err(|e| Error::from(e))?;
        let grpc_client = service_client::ServiceClient::connect(config.grpc_endpoint.clone())
            .await
            .map_err(|e| Error::from(e))?;

        Ok(Client {
            tm_client,
            grpc_client,
            config,
        })
    }

    /// Returns a block info by height.
    pub async fn get_block(&self, height: u64) -> Result<tendermint::block::Block, Error> {
        let block = self
            .tm_client
            .block(block::Height::from(height as u32))
            .await
            .map_err(|e| Error::from(e))?;

        Ok(block.block)
    }

    /// Returns a block result by height.
    pub async fn get_block_result(&self, height: u64) -> Result<block_results::Response, Error> {
        let block_result = self
            .tm_client
            .block_results(block::Height::from(height as u32))
            .await
            .map_err(|e| Error::from(e))?;

        Ok(block_result)
    }

    /// Returns a transaction by hash.
    pub async fn get_transaction(&mut self, hash: String) -> Result<GetTxResponse, Error> {
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxRequest { hash };

        let response = self
            .grpc_client
            .get_tx(request)
            .await
            .map_err(|e| Error::from(e))?;

        Ok(response.get_ref().clone())
    }

    /// Returns a transaction messages by tx hash
    /// It uses REST API, because prost cannot automatically convert it into json string
    pub async fn get_tx_messages(&mut self, hash: String) -> Result<Vec<String>, Error> {
        let url = format!(
            "{}/cosmos/tx/v1beta1/txs/{}",
            self.config.rest_api_endpoint.clone(),
            hash
        );
        let response = reqwest::get(url).await?.text().await?;

        let res: serde_json::Value = serde_json::from_str(&response)?;

        let messages = res["tx"]["body"]["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m.to_string())
            .collect();

        Ok(messages)
    }

    /// Extract events information from transactions and block.
    ///
    /// It returns list of events.
    pub fn extract_events(
        &self,
        txes: Vec<&GetTxResponse>,
        block_result: &block_results::Response,
    ) -> Vec<RawEvent> {
        let mut result: Vec<RawEvent> = vec![];
        let convert_block_events = |events: Vec<abci::Event>, tx_type: i16| {
            events
                .iter()
                .map(|evt| {
                    evt.attributes.iter().map(|attr| RawEvent {
                        tx_type: tx_type,
                        tx_hash: None,
                        event_type: evt.type_str.clone(),
                        event_key: attr.key.to_string(),
                        event_value: attr.value.to_string(),
                        indexed: false,
                    })
                })
                .flatten()
                .collect::<Vec<RawEvent>>()
        };

        let tx_events = txes
            .into_iter()
            .map(|tx| tx.tx_response.as_ref().clone().unwrap())
            .map(|tx| {
                tx.clone()
                    .events
                    .into_iter()
                    .map(|event| {
                        event
                            .clone()
                            .attributes
                            .iter()
                            .map(|attr| RawEvent {
                                tx_type: TX_TYPE_TRANSACTION,
                                tx_hash: Some(tx.txhash.clone()),
                                event_type: event.r#type.clone(),
                                event_key: from_utf8(&attr.key).unwrap().to_string(),
                                event_value: from_utf8(&attr.value).unwrap().to_string(),
                                indexed: attr.index,
                            })
                            .collect::<Vec<_>>()
                    })
                    .flatten()
            })
            .flatten()
            .collect::<Vec<_>>();

        if let Some(begin_block_events) = block_result.begin_block_events.clone() {
            let evts = convert_block_events(begin_block_events, TX_TYPE_BEGIN_BLOCK);
            result.extend(evts);
        }

        if let Some(end_block_events) = block_result.end_block_events.clone() {
            let evts = convert_block_events(end_block_events, TX_TYPE_END_BLOCK);
            result.extend(evts);
        }

        result.extend(tx_events);
        return result;
    }
}
