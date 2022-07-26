use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new("http://localhost:26657").unwrap();
    let block = client.block(Height::from(1u32)).await.unwrap();

    println!("block height at 1 =  {:?}", block);

    Ok(())
}
