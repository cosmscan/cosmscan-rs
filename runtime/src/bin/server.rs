use api_server::server::ApiServer;

#[tokio::main]
async fn main() {
    let mut server = ApiServer::new();
    server.run().await;
}