use contexter::cli::{run_cli, Cli};
use contexter::config::Config;
use contexter::server::run_server;
use env_logger::Env;
use log::info;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    // Determine the log level based on command-line arguments
    let log_level = match cli {
        Cli::Server { quiet, verbose } => {
            if quiet {
                log::LevelFilter::Error
            } else if verbose {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            }
        }
        _ => log::LevelFilter::Info,
    };

    // Initialize the logger only once with the determined log level
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

    match cli {
        Cli::Server { .. } => {
            let config = Config::load()?;
            if config.api_keys.is_empty() {
                eprintln!("No API keys defined. Please generate an API key using `contexter config generate-key <name>`.");
                return Ok(());
            }
            info!(
                "Starting server on {}:{}",
                config.listen_address, config.port
            );
            run_server(config).await?;
        }
        _ => {
            run_cli()?;
        }
    }

    Ok(())
}
