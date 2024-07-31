use crate::config::Config;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "contexter", about = "A context gathering tool for LLMs")]
pub enum Cli {
    #[structopt(name = "server", about = "Run in server mode")]
    Server {
        #[structopt(short, long, help = "Run quietly")]
        quiet: bool,

        #[structopt(short, long, help = "Verbose output")]
        verbose: bool,
    },

    #[structopt(name = "config", about = "Manage configuration")]
    Config {
        #[structopt(subcommand)]
        cmd: ConfigCommand,
    },
}

#[derive(StructOpt)]
pub enum ConfigCommand {
    #[structopt(name = "add-project", about = "Add a project")]
    AddProject {
        #[structopt(help = "Project name")]
        name: String,

        #[structopt(help = "Project path")]
        path: std::path::PathBuf,
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
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();
    let mut config = Config::load()?;

    match cli {
        Cli::Server { quiet, verbose } => {
            // TODO: Implement server logic
            println!("Running server (quiet: {}, verbose: {})", quiet, verbose);
        }
        Cli::Config { cmd } => match cmd {
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
        },
    }

    Ok(())
}
