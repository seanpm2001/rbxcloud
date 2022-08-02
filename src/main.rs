mod cli;

use clap::Parser;
use cli::Cli;
use std::process;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli_args = Cli::parse();

    match cli_args.run().await {
        Ok(str) => {
            if let Some(s) = str {
                println!("{}", s);
            }
        }
        Err(err) => {
            log::error!("{:?}", err);
            process::exit(1);
        }
    }
}
