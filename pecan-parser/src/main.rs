extern crate pecan_parser;

use std::fs::File;
use std::io::Read;
use std::env;

use pecan_parser::{Lexer, parse};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Pass a filename as argument");
        return;
    }
    let file = File::open(&args[1]);
    let mut buf = String::new();
    if let Err(e) = file.and_then(|mut f| f.read_to_string(&mut buf)) {
        println!("Error reading file: {}", e);
        return;
    }
    let lexer = Lexer::new(buf.as_str());
    match parse(lexer) {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => println!("Error parsing: {:?}", e),
    }
}
