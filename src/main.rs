use base64::{engine::general_purpose, Engine as _};
use contexter::config::Config;
use contexter::contexter::{concatenate_files, gather_relevant_files};
use contexter::server::run_server;
use env_logger::Env;
use log::{info, LevelFilter};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "contexter", about = "A context gathering tool for LLMs")]
enum Cli {
    #[structopt(name = "server", about = "Run in server mode")]
    Server {
        #[structopt(short, long, help = "Run quietly")]
        quiet: bool,

        #[structopt(short, long, help = "Verbose output")]
        verbose: bool,
    },
    #[structopt(name = "gather", about = "Gather context from files")]
    Gather {
        #[structopt(parse(from_os_str))]
        directory: PathBuf,

        #[structopt(short, long, help = "File extensions to include")]
        extensions: Vec<String>,

        #[structopt(short, long, help = "Patterns to ignore")]
        ignore: Vec<String>,
    },
    #[structopt(name = "config", about = "Manage configuration")]
    Config {
        #[structopt(subcommand)]
        cmd: ConfigCommand,
    },
}

#[derive(StructOpt)]
enum ConfigCommand {
    #[structopt(name = "add-project", about = "Add a project")]
    AddProject {
        #[structopt(help = "Project name")]
        name: String,

        #[structopt(help = "Project path")]
        path: PathBuf,
    },
    #[structopt(name = "remove-project", about = "Remove a project")]
    RemoveProject {
        #[structopt(help = "Project name")]
        name: String,
    },
    #[structopt(name = "generate-key", about = "Generate a new API key")]
    GenerateKey,
    #[structopt(name = "remove-key", about = "Remove an API key")]
    RemoveKey {
        #[structopt(help = "API key")]
        key: String,
    },
    #[structopt(name = "set-port", about = "Set the server port")]
    SetPort {
        #[structopt(help = "Port number")]
        port: u16,
    },
    #[structopt(name = "set-address", about = "Set the listen address")]
    SetAddress {
        #[structopt(help = "Listen address")]
        address: String,
    },
    #[structopt(name = "list", about = "List current configuration")]
    List,
}

fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    general_purpose::URL_SAFE_NO_PAD.encode(key)
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

fn init_logger(quiet: bool, verbose: bool) -> LevelFilter {
    let log_level = if verbose {
        LevelFilter::Debug
    } else if quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter(None, log_level)
        .init();

    log_level
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    // Initialize logger based on the command
    let log_level = match &cli {
        Cli::Server { quiet, verbose } => init_logger(*quiet, *verbose),
        _ => init_logger(false, false), // Use default logging for other commands
    };

    match cli {
        Cli::Server { quiet, verbose } => {
            let config = Config::load()?;
            run_server(config, log_level).await?;
        }
        Cli::Gather {
            directory,
            extensions,
            ignore,
        } => {
            let files = gather_relevant_files(
                directory.to_str().unwrap(),
                extensions.iter().map(AsRef::as_ref).collect(),
                ignore,
            )?;
            let (content, _) = concatenate_files(files)?;
            println!("{}", content);
        }
        Cli::Config { cmd } => {
            let mut config = Config::load()?;
            match cmd {
                ConfigCommand::AddProject { name, path } => {
                    config.add_project(name.clone(), path.clone());
                    config.save()?;
                    info!("Project '{}' added successfully with path {:?}", name, path);
                }
                ConfigCommand::RemoveProject { name } => {
                    if config.remove_project(&name).is_some() {
                        config.save()?;
                        info!("Project '{}' removed successfully", name);
                    } else {
                        println!("Project '{}' not found", name);
                    }
                }
                ConfigCommand::GenerateKey => {
                    let new_key = generate_api_key();
                    let hashed_key = hash_api_key(&new_key);
                    config.add_api_key(hashed_key);
                    config.save()?;
                    println!("New API key generated: {}", new_key);
                    println!("Please store this key securely. It won't be displayed again.");
                    info!("New API key generated successfully");
                }
                ConfigCommand::RemoveKey { key } => {
                    let hashed_key = hash_api_key(&key);
                    config.remove_api_key(&hashed_key);
                    config.save()?;
                    info!("API key removed successfully");
                }
                ConfigCommand::SetPort { port } => {
                    config.port = port;
                    config.save()?;
                    info!("Port set to {} successfully", port);
                }
                ConfigCommand::SetAddress { address } => {
                    config.listen_address = address.clone();
                    config.save()?;
                    info!("Listen address set to {} successfully", address);
                }
                ConfigCommand::List => {
                    println!("Current Configuration:");
                    println!("Port: {}", config.port);
                    println!("Listen Address: {}", config.listen_address);
                    println!("Projects:");
                    for (name, path) in &config.projects {
                        println!("  {}: {:?}", name, path);
                    }
                    println!("API Keys (hashed):");
                    for key in &config.api_keys {
                        println!("  {}", key);
                    }
                }
            }
        }
    }

    Ok(())
}
