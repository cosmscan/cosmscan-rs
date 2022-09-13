use cosmos_sdk_proto::cosmos::tx::v1beta1::{service_client, GetTxResponse};
use tendermint::block;
use tendermint_rpc::{endpoint::block_results, Client as tm_client};

use crate::errors::Error;

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
}
