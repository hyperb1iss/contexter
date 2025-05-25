use contexter::contexter::{concatenate_files, gather_relevant_files};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

fn create_test_files(dir_path: &std::path::Path) -> std::io::Result<()> {
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is test file 1.\nfn test_function() {{}}")?;

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "This is test file 2.\nstruct TestStruct {{}}")?;

    let file3_path = dir_path.join("test3.txt");
    let mut file3 = File::create(&file3_path)?;
    writeln!(file3, "This is test file 3.\nlet x = 42;")?;

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

    let files = gather_relevant_files(dir_path.to_str().unwrap(), &[], vec![])?;

    assert_eq!(files.len(), 3); // test1.txt, test2.rs, test3.txt
    assert!(files.iter().any(|f| f.ends_with("test1.txt")));
    assert!(files.iter().any(|f| f.ends_with("test2.rs")));
    assert!(files.iter().any(|f| f.ends_with("test3.txt")));
    assert!(!files.iter().any(|f| f.ends_with(".gitignore")));

    Ok(())
}

#[test]
fn test_exclusion_patterns() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    // Exclude .txt files
    let files = gather_relevant_files(
        dir_path.to_str().unwrap(),
        &[],
        vec![String::from(".*\\.txt")],
    )?;

    assert_eq!(files.len(), 1); // Only test2.rs should remain
    assert!(files.iter().any(|f| f.ends_with("test2.rs")));
    assert!(!files.iter().any(|f| f.ends_with(".txt")));

    Ok(())
}

#[test]
fn test_concatenate_files() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();
    create_test_files(dir_path)?;

    let files = gather_relevant_files(dir_path.to_str().unwrap(), &[], vec![])?;
    let (content, _) = concatenate_files(files)?;

    assert!(content.contains("Size:"));
    assert!(content.contains("Last Modified:"));
    assert!(content.contains("This is test file 1."));
    assert!(content.contains("This is test file 2."));
    assert!(content.contains("This is test file 3."));
    assert!(content.contains("Section: Documentation"));
    assert!(content.contains("Section: Source Files"));

    Ok(())
}

#[test]
fn test_file_order_and_duplicate_detection() -> std::io::Result<()> {
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create test files with some duplicate content
    let file1_path = dir_path.join("test1.txt");
    let mut file1 = File::create(&file1_path)?;
    writeln!(file1, "This is unique content in test file 1.")?;

    let file2_path = dir_path.join("test2.rs");
    let mut file2 = File::create(&file2_path)?;
    writeln!(file2, "This is unique content in test file 2.")?;

    let file3_path = dir_path.join("test3.txt");
    let mut file3 = File::create(&file3_path)?;
    writeln!(file3, "This is unique content in test file 3.")?;

    let duplicate_file_path = dir_path.join("duplicate.txt");
    let mut duplicate_file = File::create(&duplicate_file_path)?;
    writeln!(duplicate_file, "This is unique content in test file 1.")?; // Same as test1.txt

    let files = gather_relevant_files(dir_path.to_str().unwrap(), &[], vec![])?;
    let (content, filenames) = concatenate_files(files)?;

    let content_lines: Vec<&str> = content.lines().collect();

    let doc_section_index = content_lines
        .iter()
        .position(|&r| r.contains("Section: Documentation"))
        .unwrap();
    let source_section_index = content_lines
        .iter()
        .position(|&r| r.contains("Section: Source Files"))
        .unwrap();

    let file1_index = content_lines
        .iter()
        .position(|&r| r.contains("unique content in test file 1"))
        .unwrap();
    let file2_index = content_lines
        .iter()
        .position(|&r| r.contains("unique content in test file 2"))
        .unwrap();
    let file3_index = content_lines
        .iter()
        .position(|&r| r.contains("unique content in test file 3"))
        .unwrap();

    assert!(
        doc_section_index < source_section_index,
        "Documentation section should come before Source Files section"
    );
    assert!(
        file1_index < file3_index,
        "File order is incorrect: test1.txt should come before test3.txt"
    );
    assert!(
        file2_index > file1_index && file2_index > file3_index,
        "File order is incorrect: test2.rs should come after both .txt files"
    );

    // Check that duplicate content is not included
    assert_eq!(
        content_lines
            .iter()
            .filter(|&line| line.contains("unique content in test file 1"))
            .count(),
        1,
        "Duplicate content was not properly excluded"
    );

    // Check that we have the correct number of files in the output
    assert_eq!(
        filenames.len(),
        3,
        "Incorrect number of files in the output"
    );

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

    let files = gather_relevant_files(dir_path.to_str().unwrap(), &[], vec![])?;

    assert!(!files.iter().any(|f| f.ends_with("binary_file.bin")));

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

    let files = gather_relevant_files(dir_path.to_str().unwrap(), &[], vec![])?;

    assert!(!files
        .iter()
        .any(|f| f.to_str().unwrap().contains("node_modules")));
    assert!(!files.iter().any(|f| f.ends_with(".gitignore")));

    Ok(())
}
