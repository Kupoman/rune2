extern crate regex;
extern crate docopt;
extern crate rustc_serialize;

mod source_span;
mod token;
mod lexer;
mod parser;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use docopt::Docopt;
use lexer::lex_str;

// Usage documentation string
static USAGE: &'static str = "
Usage: rune [options] [<file>]
       rune --help

Options:
    -h, --help  Show this message
";


// Struct for storing command-line arguments
#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Option<String>,
    flag_help: bool,
}


fn main() {
    // Get command-line arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    
    // Open file, if valid path
    let mut f = if let Option::Some(s) = args.arg_file {
        BufReader::new(File::open(&Path::new(&s[..])).unwrap())
    }
    else {
        panic!("Invalid file path.")
    };
    
    // Read contents of file into a string
    let mut text = String::new();
    match f.read_to_string(&mut text) {
        Ok(_) => {},
        Err(_) => panic!("Failed to read file.")
    }

    // Lex the string
    let tokens = lex_str(&text[..]);
    
    // Print tokens
    //println!("{:?}", tokens);
}
