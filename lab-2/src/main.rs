use std::{process, env};
use std::fs::File;
use std::path::Path;
use std::io::{ self, Read };

use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() -> io::Result<()> {
    let mut args = env::args();

    // program name
    args.next(); 

    // supports single file now
    let file_path = match args.next() {
        Some(f) => f,
        None => { 
            eprintln!("no input files"); 
            process::exit(1);
        }
    };

    let file_path = Path::new(&file_path);
    if !file_path.is_file() {
        eprintln!("file {:?} not found", file_path);
        process::exit(1);
    }

    let mut input = String::new();
    File::open(file_path)?.read_to_string(&mut input)?;

    let lexer = Lexer::new(&input);

    match lexer.to_tokens() {
        Ok(tokens) => {
            println!("=== Tokens ===");
            for (i, t) in tokens.iter().filter(|t| !matches!(t, token::Token::EOF)).enumerate() {
                println!("({})\t{:?}", i + 1, t);
            }
            
            println!("\n=== Parsing ===");
            let mut parser = Parser::new(&tokens);
            match parser.parse() {
                Ok(func) => {
                    println!("Parsed successfully!");
                    println!("{:#?}", func);
                }
                Err(err) => {
                    eprintln!("Parse error: {}", err);
                }
            }
        }
        Err(invalid_tokens) => {
            eprintln!("invalid tokens: {:?}", invalid_tokens);
        }
    }

    return Ok(());
}
