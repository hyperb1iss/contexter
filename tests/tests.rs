use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use clipboard_anywhere::{set_clipboard, get_clipboard};
use contexter::{gather_relevant_files, concatenate_files};
use std::time::{UNIX_EPOCH, SystemTimeError};

fn system_time_error_to_io_error(err: SystemTimeError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

#[test]
fn test_concatenate_files_and_clipboard_with_metadata() -> std::io::Result<()> {
    // Create a temporary directory
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create some test files
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is a test file 1.\nfn test_function() {{}}")?;

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "This is a test file 2.\nstruct TestStruct {{}}")?;

    let file3_path = dir_path.join("test3.txt");
    let mut file3 = File::create(&file3_path)?;
    writeln!(file3, "This is a test file 1.\nfn test_function() {{}}")?; // Duplicate content of file1

    // Create a .gitignore file
    let gitignore_path = dir_path.join(".gitignore");
    let mut gitignore = File::create(&gitignore_path)?;
    writeln!(gitignore, "*.ignore")?;

    // Run the gather_relevant_files function
    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?; // No extensions, no excludes

    // Run the concatenate_files function with metadata
    let (content, filenames) = concatenate_files(files, true)?;

    // Print the actual content for debugging
    println!("Actual content:\n{}", content);

    // Extract the actual metadata (file sizes and modification times)
    let file1_metadata = std::fs::metadata(&file1_path)?;
    let file2_metadata = std::fs::metadata(&file2_path)?;
    let file1_size = file1_metadata.len();
    let file2_size = file2_metadata.len();
    let file1_modified = file1_metadata
        .modified()
        .map_err(std::io::Error::from)?
        .duration_since(UNIX_EPOCH)
        .map_err(system_time_error_to_io_error)?
        .as_secs();
    let file2_modified = file2_metadata
        .modified()
        .map_err(std::io::Error::from)?
        .duration_since(UNIX_EPOCH)
        .map_err(system_time_error_to_io_error)?
        .as_secs();

    // Construct the expected content with the correct metadata
    let expected_content = format!(
        "========================================\nSection: Source Files\n========================================\n========================================\nFile: \"{}\"\nSize: {} bytes\nLast Modified: {}\n========================================\nThis is a test file 2.\nstruct TestStruct {{}}\n========================================\nSection: Documentation\n========================================\n========================================\nFile: \"{}\"\nSize: {} bytes\nLast Modified: {}\n========================================\nThis is a test file 1.\nfn test_function() {{}}",
        file2_path.to_string_lossy(), file2_size, file2_modified,
        file1_path.to_string_lossy(), file1_size, file1_modified
    );

    println!("Expected content:\n{}", expected_content);

    // Verify the concatenated content
    assert_eq!(
        content.trim_end(), expected_content.trim_end(),
        "Content does not match expected. Actual content:\n{}",
        content
    );

    assert!(filenames.contains(&file1_path.to_string_lossy().to_string()));
    assert!(filenames.contains(&file2_path.to_string_lossy().to_string()));

    // Ensure the duplicate file content is not included
    assert!(!filenames.contains(&file3_path.to_string_lossy().to_string()));

    // Test clipboard functionality
    set_clipboard(&content).expect("Failed to set clipboard");
    let clipboard_content = get_clipboard().expect("Failed to get clipboard");

    assert_eq!(content.trim_end(), clipboard_content.trim_end());

    // Cleanup
    dir.close()?;
    Ok(())
}

#[test]
fn test_concatenate_files_and_clipboard_without_metadata() -> std::io::Result<()> {
    // Create a temporary directory
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create some test files
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is a test file 1.\nfn test_function() {{}}")?;

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "This is a test file 2.\nstruct TestStruct {{}}")?;

    let file3_path = dir_path.join("test3.txt");
    let mut file3 = File::create(&file3_path)?;
    writeln!(file3, "This is a test file 1.\nfn test_function() {{}}")?; // Duplicate content of file1

    // Create a .gitignore file
    let gitignore_path = dir_path.join(".gitignore");
    let mut gitignore = File::create(&gitignore_path)?;
    writeln!(gitignore, "*.ignore")?;

    // Run the gather_relevant_files function
    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?; // No extensions, no excludes

    // Run the concatenate_files function without metadata
    let (content, filenames) = concatenate_files(files, false)?;

    // Print the actual content for debugging
    println!("Actual content:\n{}", content);

    // Verify the concatenated content
    let expected_content = format!(
        "========================================\nSection: Source Files\n========================================\n========================================\nFile: \"{}\"\n========================================\nThis is a test file 2.\nstruct TestStruct {{}}\n========================================\nSection: Documentation\n========================================\n========================================\nFile: \"{}\"\n========================================\nThis is a test file 1.\nfn test_function() {{}}",
        file2_path.to_string_lossy(), file1_path.to_string_lossy()
    );

    println!("Expected content:\n{}", expected_content);

    assert_eq!(
        content.trim_end(), expected_content.trim_end(),
        "Content does not match expected. Actual content:\n{}",
        content
    );
    assert!(filenames.contains(&file1_path.to_string_lossy().to_string()));
    assert!(filenames.contains(&file2_path.to_string_lossy().to_string()));

    // Ensure the duplicate file content is not included
    assert!(!filenames.contains(&file3_path.to_string_lossy().to_string()));

    // Test clipboard functionality
    set_clipboard(&content).expect("Failed to set clipboard");
    let clipboard_content = get_clipboard().expect("Failed to get clipboard");

    assert_eq!(content.trim_end(), clipboard_content.trim_end());

    // Cleanup
    dir.close()?;
    Ok(())
}