use clap::Parser;
use clipboard_anywhere::set_clipboard;
use contexter::{gather_relevant_files, concatenate_files};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to search for files
    directory: String,

    /// File extensions to include
    extensions: Vec<String>,

    /// Exclude filename patterns
    #[arg(short, long)]
    exclude: Vec<String>,

    /// Copy result to clipboard
    #[arg(short, long)]
    clipboard: bool,
}

fn main() -> std::io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    let extensions: Vec<&str> = args.extensions.iter().map(AsRef::as_ref).collect();
    let excludes: Vec<&str> = args.exclude.iter().map(AsRef::as_ref).collect();

    // Gather relevant files and concatenate their content
    match gather_relevant_files(&args.directory, extensions, excludes) {
        Ok(files) => {
            match concatenate_files(files) {
                Ok((result, filenames)) => {
                    if args.clipboard {
                        for filename in filenames {
                            println!("{}", filename);
                        }
                        match set_clipboard(&result) {
                            Ok(_) => println!("The concatenated content has been copied to the clipboard."),
                            Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
                        }
                    } else {
                        println!("{}", result);
                    }
                }
                Err(e) => eprintln!("Error concatenating files: {}", e),
            }
        }
        Err(e) => eprintln!("Error gathering relevant files: {}", e),
    }

    Ok(())
}
