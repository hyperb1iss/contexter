use ignore::WalkBuilder;
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs::{metadata, read_to_string};
use std::hash::{Hash, Hasher};
use std::io;
use std::path::{Path, PathBuf};

/// Gathers relevant files from a directory based on specified extensions and exclusion patterns.
///
/// # Arguments
///
/// * `directory` - The root directory to start the search from.
/// * `extensions` - A list of file extensions to include. If empty, all files are considered.
/// * `excludes` - A list of regex patterns for files or directories to exclude.
///
/// # Returns
///
/// A Result containing a Vec of PathBuf for relevant files, or an IO error.
pub fn gather_relevant_files(
    directory: &str,
    extensions: Vec<&str>,
    excludes: Vec<String>,
) -> io::Result<Vec<PathBuf>> {
    let project_dir = PathBuf::from(directory);
    let mut relevant_files = Vec::new();

    // Combine user-provided exclusions with built-in exclusions
    let mut all_excludes = excludes;
    all_excludes.extend(vec![
        String::from(r"\.git"),
        String::from(r"\.svn"),
        String::from(r"\.hg"),
        String::from(r"\.DS_Store"),
        String::from(r"node_modules"),
        String::from(r"target"),
        String::from(r"build"),
        String::from(r"dist"),
        String::from(r"\.vscode"),
        String::from(r"\.idea"),
        String::from(r"\.vs"),
        String::from(r"package-lock\.json"),
        String::from(r"\.lock"),
    ]);

    // Create a file system walker that respects .gitignore
    let walker = WalkBuilder::new(&project_dir)
        .add_custom_ignore_filename(".gitignore")
        .build();

    // Compile exclusion patterns
    let exclude_patterns: Vec<Regex> = all_excludes
        .iter()
        .map(|pattern| Regex::new(pattern).expect("Invalid regex pattern"))
        .collect();

    // Iterate through all files in the directory
    for result in walker {
        match result {
            Ok(entry) => {
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    let path = entry.path();
                    if !is_excluded(path, &exclude_patterns)
                        && !is_likely_binary(path)?
                        && (extensions.is_empty()
                            || extensions
                                .iter()
                                .any(|ext| path.extension().and_then(|e| e.to_str()) == Some(ext)))
                    {
                        relevant_files.push(entry.into_path());
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        }
    }

    relevant_files.sort();
    Ok(relevant_files)
}

/// Checks if a file path matches any of the exclusion patterns.
fn is_excluded(path: &Path, exclude_patterns: &[Regex]) -> bool {
    let path_str = path.to_string_lossy();
    exclude_patterns.iter().any(|re| re.is_match(&path_str))
}

/// Determines if a file is likely to be binary based on its extension or content.
fn is_likely_binary(path: &Path) -> io::Result<bool> {
    // List of common binary file extensions
    const BINARY_EXTENSIONS: &[&str] = &[
        "exe", "dll", "so", "dylib", "bin", "obj", "o", "a", "lib", "pyc", "pyd", "pyo", "jpg",
        "jpeg", "png", "gif", "bmp", "tiff", "ico", "mp3", "mp4", "avi", "mov", "wmv", "flv",
        "zip", "tar", "gz", "rar", "7z", "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
    ];

    // Check if the file has a known binary extension
    if let Some(ext) = path.extension() {
        if BINARY_EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
            return Ok(true);
        }
    }

    // If not a known binary extension, check the file content for null bytes
    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0; 1024];
    let bytes_read = std::io::Read::read(&mut file, &mut buffer)?;

    Ok(buffer[..bytes_read].contains(&0))
}

/// Calculates a hash for the given value.
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// Concatenates the contents of multiple files, categorizing them and removing duplicates.
///
/// # Arguments
///
/// * `files` - A vector of PathBuf representing the files to concatenate.
///
/// # Returns
///
/// A Result containing a tuple of the concatenated content string and a vector of processed filenames.
pub fn concatenate_files(mut files: Vec<PathBuf>) -> io::Result<(String, Vec<String>)> {
    let mut content = String::new();
    let mut filenames = Vec::new();
    let mut seen_hashes = HashSet::new();

    // Sort files alphabetically by their file name
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    // Initialize strings for different file categories
    let mut config_files = String::new();
    let mut source_files = String::new();
    let mut doc_files = String::new();
    let mut test_files = String::new();

    // Process each file
    for path in files {
        let file_content = read_to_string(&path)?;
        let file_hash = calculate_hash(&file_content);

        // Only process the file if its content hasn't been seen before
        if !seen_hashes.contains(&file_hash) {
            seen_hashes.insert(file_hash);

            // Prepare file metadata
            let mut file_info = String::new();
            let metadata = metadata(&path)?;
            file_info.push_str(&format!(
                "========================================\n\
                File: {:?}\n\
                Size: {} bytes\n\
                Last Modified: {:?}\n\
                ========================================\n",
                path,
                metadata.len(),
                metadata.modified()?
            ));

            file_info.push_str(&file_content);
            file_info.push('\n');

            // Categorize the file based on its extension
            let file_ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
            match file_ext.as_str() {
                "toml" | "json" | "yaml" | "yml" => config_files.push_str(&file_info),
                "rs" => source_files.push_str(&file_info),
                "md" | "txt" => doc_files.push_str(&file_info),
                _ if file_ext.contains("test") => test_files.push_str(&file_info),
                _ => source_files.push_str(&file_info),
            }

            filenames.push(path.to_string_lossy().to_string());
        }
    }

    // Append sections in the desired order
    let sections = [
        ("Configuration Files", &config_files),
        ("Documentation", &doc_files),
        ("Source Files", &source_files),
        ("Tests", &test_files),
    ];

    // Construct the final content string
    for (section_name, section_content) in sections.iter() {
        if !section_content.is_empty() {
            content.push_str(&format!("========================================\nSection: {}\n========================================\n", section_name));
            content.push_str(section_content);
        }
    }

    Ok((content, filenames))
}