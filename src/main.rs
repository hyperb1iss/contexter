use contexter::config::Config;
use contexter::contexter::{concatenate_files, gather_relevant_files};
use contexter::server::run_server;
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
    #[structopt(name = "add-key", about = "Add an API key")]
    AddKey {
        #[structopt(help = "API key")]
        key: String,
    },
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

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    match cli {
        Cli::Server { quiet, verbose } => {
            let config = Config::load()?;
            run_server(config, quiet, verbose).await?;
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
                    config.add_project(name, path);
                    config.save()?;
                    println!("Project added successfully");
                }
                ConfigCommand::RemoveProject { name } => {
                    if config.remove_project(&name).is_some() {
                        config.save()?;
                        println!("Project removed successfully");
                    } else {
                        println!("Project not found");
                    }
                }
                ConfigCommand::AddKey { key } => {
                    config.add_api_key(key);
                    config.save()?;
                    println!("API key added successfully");
                }
                ConfigCommand::RemoveKey { key } => {
                    config.remove_api_key(&key);
                    config.save()?;
                    println!("API key removed successfully");
                }
                ConfigCommand::SetPort { port } => {
                    config.port = port;
                    config.save()?;
                    println!("Port set successfully");
                }
                ConfigCommand::SetAddress { address } => {
                    config.listen_address = address;
                    config.save()?;
                    println!("Listen address set successfully");
                }
                ConfigCommand::List => {
                    println!("Current Configuration:");
                    println!("Port: {}", config.port);
                    println!("Listen Address: {}", config.listen_address);
                    println!("Projects:");
                    for (name, path) in &config.projects {
                        println!("  {}: {:?}", name, path);
                    }
                    println!("API Keys:");
                    for key in &config.api_keys {
                        println!("  {}", key);
                    }
                }
            }
        }
    }

    Ok(())
}
