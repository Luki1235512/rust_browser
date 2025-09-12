use rust_browser::{URL, load};

mod utils;
use utils::create_test_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let url = if args.len() > 1 {
        URL::new(args[1].clone())
    } else {
        create_test_file()?;
        URL::default_file()
    };

    load(url)?;
    Ok(())
}
