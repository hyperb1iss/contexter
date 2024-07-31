use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use clipboard_anywhere::{set_clipboard, get_clipboard};
use contexter::{gather_relevant_files, concatenate_files};

fn create_test_files(dir_path: &std::path::Path) -> std::io::Result<()> {
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is a test file 1.\nfn test_function() {{}}")?;

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "This is a test file 2.\nstruct TestStruct {{}}")?;

    let file3_path = dir_path.join("test3.txt");
    let mut file3 = File::create(&file3_path)?;
    writeln!(file3, "This is a test file 1.\nfn test_function() {{}}")?; // Duplicate content of file1

    let gitignore_path = dir_path.join(".gitignore");
    let mut gitignore = File::create(&gitignore_path)?;
    writeln!(gitignore, "*.ignore")?;

    Ok(())
}

#[test]
fn test_gather_relevant_files_basic() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    
    assert_eq!(files.len(), 3); // test1.txt, test2.rs, test3.txt
    assert!(files.iter().any(|f| f.ends_with("test1.txt")));
    assert!(files.iter().any(|f| f.ends_with("test2.rs")));
    assert!(files.iter().any(|f| f.ends_with("test3.txt")));
    assert!(!files.iter().any(|f| f.ends_with(".gitignore")));

    Ok(())
}

#[test]
fn test_concatenate_files_with_metadata() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    let (content, _) = concatenate_files(files, true)?;

    assert!(content.contains("Size:"));
    assert!(content.contains("Last Modified:"));
    assert!(content.contains("This is a test file 1."));
    assert!(content.contains("This is a test file 2."));

    Ok(())
}

#[test]
fn test_concatenate_files_without_metadata() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    let (content, _) = concatenate_files(files, false)?;

    assert!(!content.contains("Size:"));
    assert!(!content.contains("Last Modified:"));
    assert!(content.contains("This is a test file 1."));
    assert!(content.contains("This is a test file 2."));

    Ok(())
}

#[test]
fn test_file_order_and_sections() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    let (content, _) = concatenate_files(files, false)?;

    let content_lines: Vec<&str> = content.lines().collect();
    let file1_index = content_lines.iter().position(|&r| r.contains("test1.txt")).unwrap();
    let file2_index = content_lines.iter().position(|&r| r.contains("test2.rs")).unwrap();
    
    assert!(file2_index < file1_index, "File order is incorrect");
    assert!(content.contains("Section: Source Files"));
    assert!(content.contains("Section: Documentation"));

    Ok(())
}

#[test]
fn test_exclusion_patterns() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    // Exclude .txt files
    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![".*\\.txt"])?;
    
    assert_eq!(files.len(), 1); // Only test2.rs should remain
    assert!(files.iter().any(|f| f.ends_with("test2.rs")));
    assert!(!files.iter().any(|f| f.ends_with(".txt")));

    Ok(())
}

#[test]
fn test_built_in_exclusions() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    // Create a file that should be excluded by default
    let node_modules_path = dir_path.join("node_modules");
    std::fs::create_dir(&node_modules_path)?;
    File::create(node_modules_path.join("package.json"))?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    
    assert!(!files.iter().any(|f| f.to_str().unwrap().contains("node_modules")));
    assert!(!files.iter().any(|f| f.ends_with(".gitignore")));

    Ok(())
}

#[test]
fn test_binary_file_skipping() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    // Create a binary file
    let binary_file_path = dir_path.join("binary_file.bin");
    let mut binary_file = File::create(&binary_file_path)?;
    binary_file.write_all(&[0u8; 1024])?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    
    assert!(!files.iter().any(|f| f.ends_with("binary_file.bin")));

    Ok(())
}

#[test]
fn test_clipboard_functionality() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), vec![], vec![])?;
    let (content, _) = concatenate_files(files, false)?;

    set_clipboard(&content).expect("Failed to set clipboard");
    let clipboard_content = get_clipboard().expect("Failed to get clipboard");
    
    assert_eq!(content.trim(), clipboard_content.trim(), "Clipboard content does not match");

    Ok(())
}