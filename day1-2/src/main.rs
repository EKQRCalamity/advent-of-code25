use std::{fs::OpenOptions, path::PathBuf};

use crate::{args::ArgParser, parser::{RotationParser, RotationToken}, rotator::Rotator};

mod parser;
mod args;
mod rotator;

fn parse_file(path: PathBuf) -> Result<(String, Vec<RotationToken>), std::io::Error> {
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

    let mut parser = RotationParser::new(&buffer);

    let tokens = parser.tokenize_input();

    return Ok((buffer, tokens));
}

const ROTATOR_INITAL: u16 = 50;

fn main() {
    let argparser = ArgParser::new();

    println!("File: {}", &argparser.file.display());

    let (_buffer, tokens) = match parse_file(argparser.file) {
        Ok(x) => x,
        Err(e) => panic!("Parse file error: {}", e),
    };

    let mut rotator = Rotator::new(ROTATOR_INITAL);
    let mut old_pos = ROTATOR_INITAL;

    let mut zeros = 0;
    for token in tokens {
        let (new_pos, _zeros) = rotator.rotate(&token);
        zeros += _zeros;
        println!("Old: {} New: {} Token: {}", old_pos, new_pos, token);
        old_pos = new_pos;
    }
    println!("Password: {}", zeros);
}
