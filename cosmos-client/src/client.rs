use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client;

use tendermint::block;
use tendermint_rpc::Client as tm_client;

use crate::{
    bytes_to_tx_hash, convert_block_events,
    errors::Error,
    response::{self, EventType},
};

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
    pub async fn get_block(&self, height: i64) -> Result<(response::Block, Vec<String>), Error> {
        let block = self
            .tm_client
            .block(block::Height::from(height as u32))
            .await
            .map_err(|e| Error::from(e))?;

        let tx_hashes = block
            .block
            .data
            .iter()
            .map(bytes_to_tx_hash)
            .collect::<Vec<_>>();

        let resp = response::Block::from(block.block);

        Ok((resp, tx_hashes))
    }

    /// Returns a block result by height.
    pub async fn get_block_result(&self, height: i64) -> Result<response::BlockResult, Error> {
        let block_result = self
            .tm_client
            .block_results(block::Height::from(height as u32))
            .await
            .map_err(|e| Error::from(e))?;

        let mut resp = response::BlockResult {
            height,
            begin_block_events: vec![],
            end_block_events: vec![],
        };

        if let Some(events) = block_result.begin_block_events {
            let converted = convert_block_events(events, height, EventType::BeginBlock);
            resp.begin_block_events = converted;
        }

        if let Some(events) = block_result.end_block_events {
            let converted = convert_block_events(events, height, EventType::EndBlock);
            resp.end_block_events = converted;
        }

        Ok(resp)
    }

    /// Returns a transaction by hash.
    pub async fn get_transaction(
        &mut self,
        hash: String,
    ) -> Result<(response::Transaction, Vec<response::Event>), Error> {
        let request = cosmos_sdk_proto::cosmos::tx::v1beta1::GetTxRequest { hash };

        let response = self
            .grpc_client
            .get_tx(request)
            .await
            .map_err(|e| Error::from(e))?;

        let resp = response::Transaction::from(&response.get_ref().clone());

        let mut events: Vec<response::Event> = vec![];
        if let Some(tx_resp) = &response.get_ref().tx_response {
            for log in tx_resp.logs.iter() {
                for (seq, evt) in log.events.iter().enumerate() {
                    for attr in evt.attributes.iter() {
                        let raw_event = response::Event {
                            tx_type: EventType::Transaction,
                            tx_hash: Some(tx_resp.txhash.clone()),
                            block_height: tx_resp.height,
                            event_seq: seq as i32,
                            event_type: evt.r#type.clone(),
                            event_key: attr.key.clone(),
                            event_value: attr.value.clone(),
                            indexed: false,
                        };
                        events.push(raw_event);
                    }
                }
            }
        }

        Ok((resp, events))
    }

    /// Returns a transaction messages by tx hash
    /// It uses REST API, because prost cannot automatically convert it into json string
    pub async fn get_tx_messages(&self, hash: String) -> Result<Vec<String>, Error> {
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
}
