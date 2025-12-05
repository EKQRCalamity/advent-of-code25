use std::{fs::OpenOptions, path::PathBuf};

use crate::{args::ArgParser, parser::{RangeParser, RangeToken}, validator::Validator};

mod parser;
mod args;
mod validator;

fn parse_file(path: PathBuf) -> Result<Vec<RangeToken>, std::io::Error> {
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

    let mut parser = RangeParser::new(&buffer);

    let tokens = parser.tokenize_input();

    return Ok(tokens);
}

fn main() {
    let argparser = ArgParser::new();

    println!("File: {}", &argparser.file.display());

    let tokens = match parse_file(argparser.file) {
        Ok(x) => x,
        Err(e) => panic!("Parse file error: {}", e),
    };

    let mut value = 0;

    for token in tokens {
        let invalids = Validator::validate_token(&token);

        if let RangeToken::Range(lower, upper) = token {
            println!("{}-{} {}: {} = {}", lower, upper, invalids.len(), invalids.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" + "), invalids.iter().sum::<u64>());
            value += invalids.iter().sum::<u64>();
        }
    }

    println!("Password should be: {}", value);
}
