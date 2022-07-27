use clap::Parser;
use runtime::config::{Config};
use runtime::fetcher::FetcherApp;

#[derive(Parser)]
#[clap(author,version,about)]
struct Cli {
    #[clap(short, long, value_parser)]
    filename: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let cli:Cli = Cli::parse();
    let config = match Config::from_file(cli.filename.clone()) {
        Ok(_config) => _config,
        Err(e) => panic!("wrong config file location: {}, err: {}", cli.filename, e),
    };

    let fetcher = FetcherApp::new(config.fetcher);
    fetcher.start().await;
}
