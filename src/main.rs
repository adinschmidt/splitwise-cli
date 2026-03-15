use clap::Parser;

use splitwise_cli::cli::Cli;
use splitwise_cli::config::Config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match Config::from_cli(&cli).and_then(|config| splitwise_cli::run(cli, config)) {
        Ok(future) => {
            if let Err(error) = future.await {
                eprintln!("{error}");
                std::process::exit(error.exit_code());
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.exit_code());
        }
    }
}
