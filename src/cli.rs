use crate::cli_handlers;
use crate::config::Config;
use std::path::PathBuf;
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
pub enum ConfigCommand {
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

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();
    let mut config = Config::load()?;

    match cli {
        Cli::Server {
            quiet: _,
            verbose: _,
        } => {
            // Server logic will be handled in main.rs
            Ok(())
        }
        Cli::Gather {
            directory,
            extensions,
            ignore,
        } => cli_handlers::handle_gather(directory, extensions, ignore),
        Cli::Config { cmd } => match cmd {
            ConfigCommand::AddProject { name, path } => {
                cli_handlers::handle_config_add_project(&mut config, name, path)
            }
            ConfigCommand::RemoveProject { name } => {
                cli_handlers::handle_config_remove_project(&mut config, name)
            }
            ConfigCommand::GenerateKey => cli_handlers::handle_config_generate_key(&mut config),
            ConfigCommand::RemoveKey { key } => {
                cli_handlers::handle_config_remove_key(&mut config, key)
            }
            ConfigCommand::SetPort { port } => {
                cli_handlers::handle_config_set_port(&mut config, port)
            }
            ConfigCommand::SetAddress { address } => {
                cli_handlers::handle_config_set_address(&mut config, address)
            }
            ConfigCommand::List => {
                cli_handlers::handle_config_list(&config);
                Ok(())
            }
        },
    }
}
