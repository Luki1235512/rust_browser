use std::{env, fs};

pub fn create_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let test_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>RustBrowser Test Page</title>
</head>
<body>
    <h1>Welcome to RustBrowser!</h1>
    <p>This is a test HTML file for the RustBrowser.</p>
    <p>You can edit this file to test different HTML content.</p>
    <ul>
        <li>First item</li>
        <li>Second item</li>
        <li>Third item</li>
    </ul>
</body>
</html>"#;

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let tmp_dir = current_dir.join("tmp");
    let test_file_path = tmp_dir.join("test.html");

    fs::create_dir_all(&tmp_dir)?;

    fs::write(&test_file_path, test_content)?;
    println!("Created test file at: {}", test_file_path.display());
    Ok(())
}
