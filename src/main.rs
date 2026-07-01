use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: useless <file.usl>");
        return;
    }

    let filename = &args[1];

    if !is_valid_file(filename) {
        eprintln!("Error: only .usl files are allowed");
        return;
    }

    let contents = fs::read_to_string(filename).expect("Could not read file");

    println!("Running:\n{}", contents);
}

fn is_valid_file(filename: &str) -> bool {
    Path::new(filename).extension().and_then(|ext| ext.to_str()) == Some("usl")
}
