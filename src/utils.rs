use std::collections::HashSet;
use std::fs::{read_to_string, metadata};
use std::io;
use std::path::PathBuf;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use ignore::WalkBuilder;
use regex::Regex;
use std::time::SystemTime;

pub fn gather_relevant_files(directory: &str, extensions: Vec<&str>, excludes: Vec<&str>) -> io::Result<Vec<PathBuf>> {
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
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Concatenate the full contents of the files, skipping duplicates based on content hash
pub fn concatenate_files(files: Vec<PathBuf>) -> io::Result<(String, Vec<String>)> {
    let mut content = String::new();
    let mut filenames = Vec::new();
    let mut seen_hashes = HashSet::new();

    // Section headers
    content.push_str("========================================\nSection: Configuration Files\n========================================\n");

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

            // Add metadata
            let metadata = metadata(&path)?;
            let file_size = metadata.len();
            let modified = metadata.modified().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let modified_date = modified.duration_since(SystemTime::UNIX_EPOCH).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.as_secs();

            content.push_str(&format!("========================================\nFile: {:?}\nSize: {} bytes\nLast Modified: {}\n========================================\n", path, file_size, modified_date));
            content.push_str(&file_content.trim_end());
            filenames.push(path.to_string_lossy().to_string());
        }
    }

    Ok((content, filenames))
}
