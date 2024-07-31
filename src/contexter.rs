use std::collections::HashSet;
use std::fs::{read_to_string, metadata};
use std::io;
use std::path::PathBuf;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;
use ignore::WalkBuilder;
use regex::Regex;
use std::time::{UNIX_EPOCH, SystemTimeError};

fn system_time_error_to_io_error(err: SystemTimeError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub fn gather_relevant_files(directory: &str, extensions: Vec<&str>, excludes: Vec<&str>) -> io::Result<Vec<PathBuf>> {
    let project_dir = PathBuf::from(directory);
    let mut relevant_files = Vec::new();

    let walker = WalkBuilder::new(&project_dir)
        .add_custom_ignore_filename(".gitignore")
        .build();

    let exclude_patterns: Vec<Regex> = excludes.iter()
        .map(|pattern| Regex::new(pattern).expect("Invalid regex pattern"))
        .collect();

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

    relevant_files.sort();
    Ok(relevant_files)
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn concatenate_files(files: Vec<PathBuf>, include_metadata: bool) -> io::Result<(String, Vec<String>)> {
    let mut content = String::new();
    let mut filenames = Vec::new();
    let mut seen_hashes = HashSet::new();

    let mut config_files = String::new();
    let mut source_files = String::new();
    let mut doc_files = String::new();
    let mut test_files = String::new();

    for path in files {
        let file_content = match read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file {:?}: {}", path, err);
                continue;
            }
        };

        let file_hash = calculate_hash(&file_content);

        if !seen_hashes.contains(&file_hash) {
            seen_hashes.insert(file_hash);

            let mut file_info = String::new();

            if include_metadata {
                let metadata = metadata(&path)?;
                let file_size = metadata.len();
                let modified = metadata.modified()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
                    .duration_since(UNIX_EPOCH)
                    .map_err(system_time_error_to_io_error)?
                    .as_secs();
                file_info.push_str(&format!("========================================\nFile: {:?}\nSize: {} bytes\nLast Modified: {}\n========================================\n", path, file_size, modified));
            } else {
                file_info.push_str(&format!("========================================\nFile: {:?}\n========================================\n", path));
            }

            file_info.push_str(&file_content.trim_end());

            let file_ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
            if file_ext == "toml" || file_ext == "json" || file_ext == "yaml" || file_ext == "yml" {
                config_files.push_str(&file_info);
                config_files.push('\n');
            } else if file_ext == "rs" {
                source_files.push_str(&file_info);
                source_files.push('\n');
            } else if file_ext == "md" || file_ext == "txt" {
                doc_files.push_str(&file_info);
                doc_files.push('\n');
            } else if file_ext == "test" {
                test_files.push_str(&file_info);
                test_files.push('\n');
            } else {
                source_files.push_str(&file_info);
                source_files.push('\n');
            }

            filenames.push(path.to_string_lossy().to_string());
        }
    }

    if !config_files.is_empty() {
        content.push_str("========================================\nSection: Configuration Files\n========================================\n");
        content.push_str(&config_files);
    }

    if !source_files.is_empty() {
        content.push_str("========================================\nSection: Source Files\n========================================\n");
        content.push_str(&source_files);
    }

    if !doc_files.is_empty() {
        content.push_str("========================================\nSection: Documentation\n========================================\n");
        content.push_str(&doc_files);
    }

    if !test_files.is_empty() {
        content.push_str("========================================\nSection: Tests\n========================================\n");
        content.push_str(&test_files);
    }

    Ok((content, filenames))
}
