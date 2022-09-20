use api_server::{server::ApiServer, Config};
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser)]
    filename: String,
}

#[tokio::main]
async fn main() {
    // initialize logger
    env_logger::init();

    // parse command line flags
    let cli: Cli = Cli::parse();
    let config = Config::from_file(cli.filename.clone())
        .unwrap_or_else(|_| panic!("wrong config file location: {}", cli.filename));

    let server = ApiServer::new(config);
    match server.run().await {
        Ok(_) => log::info!("server has been stopped"),
        Err(e) => log::error!("server stopped unexpectedly {}", e),
    };
}
