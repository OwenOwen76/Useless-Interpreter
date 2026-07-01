mod compiler;
mod lexer;
mod vm;

use crate::lexer::*;
use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: useless <file.usl>");
        return Ok(());
    }

    let filename = &args[1];

    if !is_valid_file(filename) {
        eprintln!("Error: only .usl files are allowed");
        return Ok(());
    }

    let contents = fs::read_to_string(filename)?;

    let lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize();

    println!("{:#?}", contents);
    println!("{:#?}", tokens);

    Ok(())
}

fn is_valid_file(filename: &str) -> bool {
    Path::new(filename).extension().and_then(|ext| ext.to_str()) == Some("usl")
}
