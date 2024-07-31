mod server;
use contexter::{gather_relevant_files, concatenate_files};


use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "s", long = "server")]
    server: bool,

    #[structopt(short = "k", long = "key", default_value = "")]
    key: String,

    #[structopt(short = "d", long = "directory")]
    directory: Option<String>,

    #[structopt(short = "e", long = "extensions", use_delimiter = true)]
    extensions: Vec<String>,

    #[structopt(short = "x", long = "exclude_patterns", use_delimiter = true)]
    exclude_patterns: Vec<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    if args.server {
        server::run_server(args.key).await
    } else {
        if let Some(directory) = args.directory {
            let extensions: Vec<&str> = args.extensions.iter().map(String::as_str).collect();
            let exclude_patterns: Vec<&str> = args.exclude_patterns.iter().map(String::as_str).collect();
            let relevant_files = gather_relevant_files(&directory, extensions, exclude_patterns)?;
            let concatenated_files = concatenate_files(relevant_files, true).unwrap();
            println!("{:?}", concatenated_files);
        }
        Ok(())
    }
}
