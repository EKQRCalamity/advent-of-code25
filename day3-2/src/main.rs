use std::{fs::OpenOptions, path::PathBuf};

use crate::{args::ArgParser, charge_finder::Finder, parser::{BankParser, BankToken}};

mod parser;
mod args;
mod charge_finder;

fn parse_file(path: PathBuf) -> Result<Vec<BankToken>, std::io::Error> {
    use std::io::Read;
    if !path.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File does not exist."));
    }

    if !path.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Path is not a file."));
    }

    let mut buffer: String = String::new();

    OpenOptions::new()
        .read(true)
        .open(path.clone())?
        .read_to_string(&mut buffer)?;

    let mut parser = BankParser::new(&buffer);

    let tokens = parser.tokenize_input();

    return Ok(tokens);
}

fn main() {
    let argparser = ArgParser::new();

    let tokens = match parse_file(argparser.file) {
        Ok(x) => x,
        Err(e) => panic!("Parse file error: {}", e),
    };

    let mut value = 0;

    for token in tokens {
        let max = Finder::find(&token);
        value += max;

        if let BankToken::Battery(x) = token {
            println!("{} => {} = {}", x, max, value);
        }
    }

    println!("Password should be: {}", value);
}
