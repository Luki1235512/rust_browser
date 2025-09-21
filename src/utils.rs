use std::{env, fs};

pub fn create_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let test_content = r#"<html>
    <body>
        <div>This is a simple</div>
        <div>web page with some</div>
        <span>text in it.</span>
    </body>
</html>"#;

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let some_dir = current_dir.join("some").join("directory");
    let test_file_path = some_dir.join("example1-simple.html");

    fs::create_dir_all(&some_dir)?;

    fs::write(&test_file_path, test_content)?;
    println!("Created test file at: {}", test_file_path.display());
    Ok(())
}
