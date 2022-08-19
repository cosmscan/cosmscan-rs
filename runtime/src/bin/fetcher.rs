use clap::Parser;
use env_logger::Env;
use log::{error, info};
use runtime::config::Config;
use runtime::fetcher::FetcherApp;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser)]
    filename: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli: Cli = Cli::parse();
    let config = Config::from_file(cli.filename.clone())
        .unwrap_or_else(|_| panic!("wrong config file location: {}", cli.filename));

    let fetcher = FetcherApp::new(config);
    match fetcher.start().await {
        Ok(_) => {
            info!("fetcher app finished");
        }
        Err(e) => {
            error!("unexpected error during fetching blockchain: {:?}", e);
            panic!("teardown the fetcher");
        }
    }
}
  