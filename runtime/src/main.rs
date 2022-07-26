#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("http://localhost:26657/block?height=1")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    println!("Hello, world!");

    Ok(())
}
