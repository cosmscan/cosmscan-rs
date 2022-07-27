mod config;

use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() {
    let mut block_number = Height::from(1u32);
    let client = HttpClient::new("http://localhost:26657").unwrap();

    loop {
        let block = client.block(block_number).await;
        match block {
            Ok(res) => {
                println!("block height at {} = {:?}", block_number, res);
                block_number = Height::from(block_number.value().checked_add(1));
            },
            Err(err) => {
                println!("error occurred with {}", err);
            }
        }
    }
}
