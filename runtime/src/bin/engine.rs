use clap::Parser;
use cosmscan_engine::{app::App, config::Config};

use log::{error, info};

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

    // start a fetcher
    let fetcher = App::new(config).await.unwrap();
    match fetcher.start().await {
        Ok(_) => {
            info!("engine app finished");
        }
        Err(e) => {
            error!("unexpected error during fetching blockchain: {:?}", e);
            panic!("teardown the engine");
        }
    }
}
