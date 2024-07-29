use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self};
use std::path::PathBuf;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use clap::Parser;
use clipboard_anywhere::set_clipboard;
use ignore::WalkBuilder;
use regex::Regex;

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

/// Gather files from the specified directory based on the provided extensions and exclusions
fn gather_relevant_files(directory: &str, extensions: Vec<&str>, excludes: Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let project_dir = PathBuf::from(directory);
    let mut relevant_files = Vec::new();

    // Walk the directory considering .gitignore
    let walker = WalkBuilder::new(&project_dir)
        .add_custom_ignore_filename(".gitignore")
        .build();

    // Compile exclusion patterns into regular expressions
    let exclude_patterns: Vec<Regex> = excludes.iter()
        .map(|pattern| Regex::new(pattern).expect("Invalid regex pattern"))
        .collect();

    // Collect relevant files based on extensions and exclusions
    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    if !exclude_patterns.iter().any(|re| re.is_match(&path.to_string_lossy())) &&
                       (extensions.is_empty() || extensions.iter().any(|ext| path.extension().and_then(|e| e.to_str()) == Some(ext))) {
                        relevant_files.push(entry.into_path());
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                if let Some(inner_err) = err.io_error() {
                    if inner_err.kind() == io::ErrorKind::PermissionDenied {
                        eprintln!("Permission denied while accessing {:?}", err);
                    }
                } else {
                    eprintln!("Other error occurred: {:?}", err);
                }
            }
        }
    }

    // Ensure consistent output order
    relevant_files.sort();
    Ok(relevant_files)
}

/// Calculate a hash value for a given object to identify duplicates
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Concatenate the contents of the files, skipping duplicates based on content hash
fn concatenate_files(files: Vec<PathBuf>) -> io::Result<(String, Vec<String>)> {
    let mut content = String::new();
    let mut filenames = Vec::new();
    let mut seen_hashes = HashSet::new();

    for path in files {
        let file_content = match read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file {:?}: {}", path, err);
                continue;
            }
        };
        let file_hash = calculate_hash(&file_content);

        // Skip duplicate content
        if !seen_hashes.contains(&file_hash) {
            seen_hashes.insert(file_hash);

            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&format!("========================================\nFile: {:?}\n========================================\n", path));
            content.push_str(&file_content.trim_end());
            filenames.push(path.to_string_lossy().to_string());
        }
    }

    Ok((content, filenames))
}

fn main() -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use clipboard_anywhere::{set_clipboard, get_clipboard};

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

        let file3_path = dir_path.join("test3.txt");
        let mut file3 = File::create(&file3_path)?;
        writeln!(file3, "This is a test file 1.")?; // Duplicate content of file1

        // Create a .gitignore file
        let gitignore_path = dir_path.join(".gitignore");
        let mut gitignore = File::create(&gitignore_path)?;
        writeln!(gitignore, "*.ignore")?;

        // Run the gather_relevant_files function
        let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?; // No extensions, no excludes

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

        // Ensure the duplicate file content is not included
        assert!(!filenames.contains(&file3_path.to_string_lossy().to_string()));

        // Test clipboard functionality
        set_clipboard(&content).expect("Failed to set clipboard");
        let clipboard_content = get_clipboard().expect("Failed to get clipboard");

        assert_eq!(content, clipboard_content);

        // Cleanup
        dir.close()?;
        Ok(())
    }
}
