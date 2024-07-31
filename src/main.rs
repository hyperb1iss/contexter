use contexter::config::Config;
use contexter::server::run_server;
use contexter::cli::{Cli, run_cli};
use log::info;
use env_logger::Env;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::from_args();

    match cli {
        Cli::Server { quiet, verbose } => {
            let log_level = if quiet {
                log::LevelFilter::Error
            } else if verbose {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            };
            env_logger::Builder::from_env(Env::default().default_filter_or(log_level.to_string())).init();

            let config = Config::load()?;
            info!("Starting server on {}:{}", config.listen_address, config.port);
            run_server(config).await?;
        },
        _ => {
            run_cli()?;
        },
    }

    Ok(())
}