use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use clap::Parser;
use clipboard_anywhere::{set_clipboard, get_clipboard};
use ignore::WalkBuilder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to search for files
    directory: String,

    /// File extensions to include
    extensions: Vec<String>,

    /// Copy result to clipboard
    #[arg(short, long)]
    clipboard: bool,
}

fn gather_relevant_files(directory: &str, extensions: Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let project_dir = PathBuf::from(directory);
    let mut relevant_files = Vec::new();

    let walker = WalkBuilder::new(&project_dir)
        .add_custom_ignore_filename(".gitignore")
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if extensions.is_empty() || extensions.iter().any(|ext| entry.path().extension().and_then(|e| e.to_str()) == Some(ext)) {
                        relevant_files.push(entry.into_path());
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        }
    }

    Ok(relevant_files)
}

fn concatenate_files(files: Vec<PathBuf>) -> io::Result<(String, Vec<String>)> {
    let mut content = String::new();
    let mut filenames = Vec::new();

    for path in files {
        let mut file_content = String::new();
        let mut file = File::open(&path)?;
        file.read_to_string(&mut file_content)?;
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("========================================\nFile: {:?}\n========================================\n", path));
        content.push_str(&file_content.trim_end());
        filenames.push(path.to_string_lossy().to_string());
    }

    Ok((content, filenames))
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let extensions: Vec<&str> = args.extensions.iter().map(AsRef::as_ref).collect();

    match gather_relevant_files(&args.directory, extensions) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_concatenate_files_and_clipboard() -> io::Result<()> {
        // Create a temporary directory
        let dir = tempdir()?;
        let dir_path = dir.path();

        // Create some test files
        let file1_path = dir_path.join("test1.txt");
        let mut file1 = File::create(&file1_path)?;
        writeln!(file1, "This is a test file 1.")?;

        let file2_path = dir_path.join("test2.txt");
        let mut file2 = File::create(&file2_path)?;
        writeln!(file2, "This is a test file 2.")?;

        // Create a .gitignore file
        let gitignore_path = dir_path.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path)?;
        writeln!(gitignore, "*.ignore")?;

        // Run the gather_relevant_files function
        let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![])?; // No extensions

        // Run the concatenate_files function
        let (content, filenames) = concatenate_files(files)?;

        // Verify the concatenated content
        let expected_content1 = format!(
            "========================================\nFile: {:?}\n========================================\nThis is a test file 1.\n========================================\nFile: {:?}\n========================================\nThis is a test file 2.",
            file1_path, file2_path
        );

        let expected_content2 = format!(
            "========================================\nFile: {:?}\n========================================\nThis is a test file 2.\n========================================\nFile: {:?}\n========================================\nThis is a test file 1.",
            file2_path, file1_path
        );

        assert!(content == expected_content1 || content == expected_content2);
        assert!(filenames.contains(&file1_path.to_string_lossy().to_string()));
        assert!(filenames.contains(&file2_path.to_string_lossy().to_string()));

        // Test clipboard functionality
        set_clipboard(&content).expect("Failed to set clipboard");
        let clipboard_content = get_clipboard().expect("Failed to get clipboard");

        assert_eq!(content, clipboard_content);

        // Cleanup
        dir.close()?;
        Ok(())
    }
}
