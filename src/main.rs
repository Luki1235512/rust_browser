use rust_browser::{URL, load};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln! {"Usage: {} <url>", args[0]};
        std::process::exit(1);
    }

    let url = URL::new(args[1].clone());
    match load(url) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
