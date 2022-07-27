use std::thread::sleep;
use std::time::Duration;
use tendermint::block::Height;
use tendermint_rpc::{Client, HttpClient};
use clap::Parser;
use runtime::config::{Config, FetcherConfig};

#[derive(Parser)]
#[clap(author,version,about)]
struct Cli {
    #[clap(short, long, value_parser)]
    filename: String,
}

#[tokio::main]
async fn main() {
    let cli:Cli = Cli::parse();
    println!("{}", cli.filename);

    let config = match Config::from_file(cli.filename.clone()) {
        Ok(_config) => _config,
        Err(e) => panic!("wrong config file location: {}, err: {}", cli.filename, e),
    };

    println!("read from config: {:?}", config);
}
