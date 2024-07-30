use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use clipboard_anywhere::{set_clipboard, get_clipboard};
use contexter::{gather_relevant_files, concatenate_files};

#[test]
fn test_concatenate_files_and_clipboard() -> std::io::Result<()> {
    // Create a temporary directory
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create some test files
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is a test file 1.\nfn test_function() {{}}")?;

    let file2_path = dir_path.join("test2.txt");
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

    // Run the concatenate_files function
    let (content, filenames) = concatenate_files(files)?;

    // Print the actual content for debugging
    println!("Actual content:\n{}", content);

    // Verify the concatenated content
    let expected_content1 = format!(
        "========================================\nFile: \"{}\"\n========================================\nThis is a test file 1.\nfn test_function() {{}}\n========================================\nFile: \"{}\"\n========================================\nThis is a test file 2.\nstruct TestStruct {{}}",
        file1_path.to_string_lossy(), file2_path.to_string_lossy()
    );

    let expected_content2 = format!(
        "========================================\nFile: \"{}\"\n========================================\nThis is a test file 2.\nstruct TestStruct {{}}\n========================================\nFile: \"{}\"\n========================================\nThis is a test file 1.\nfn test_function() {{}}",
        file2_path.to_string_lossy(), file1_path.to_string_lossy()
    );

    assert!(content == expected_content1 || content == expected_content2, "Content does not match expected. Actual content:\n{}", content);
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
