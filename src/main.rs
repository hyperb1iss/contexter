use contexter::config::Config;
use contexter::server::run_server;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "contexter", about = "A context gathering tool for LLMs")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "server", about = "Run in server mode")]
    Server {
        #[structopt(short, long, help = "Run quietly")]
        quiet: bool,

        #[structopt(short, long, help = "Verbose output")]
        verbose: bool,
    },
    // Add other commands here as needed
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::from_args();

    match cli.cmd {
        Command::Server { quiet, verbose } => {
            let config = Config::load().expect("Failed to load configuration");
            run_server(config, quiet, verbose).await?;
        } // Handle other commands here
    }

    Ok(())
}
