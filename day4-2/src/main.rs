use std::{fs::OpenOptions, path::PathBuf, time::Duration};

use crate::{args::ArgParser, parser::{Lines, WorldParser}, world::World};

mod args;
mod parser;
mod world;

fn parse_file(path: PathBuf) -> Result<Lines, std::io::Error> {
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

    let mut parser = WorldParser::new(&buffer);

    let tokens: Lines = parser.tokenize_input();

    return Ok(tokens);
}

fn main() {
    let argparser = ArgParser::new();

    let tokens = match parse_file(argparser.file) {
        Ok(x) => x,
        Err(e) => panic!("Parse file error: {}", e),
    };

    let mut value = 0;

    let mut world = World::new(tokens);

    println!("World: \n{}", world.display());
    std::thread::sleep(Duration::from_millis(500));
    loop {
        let mut removed = false;
        loop {
            if world.current_is_paper() {
                let papers_around = world.get_papers_around_position();
                if papers_around < 4 {
                    world.remove_current_paper();
                    removed = true;
                    value += 1;
                }
            }
        
            if world.should_end() {
                break;
            }
        
            world.next();
        }
        println!("{}", (0..=136).map(|_| "-".to_owned()).collect::<String>());
        println!("World: {}", (0..=129).map(|_| "#".to_owned()).collect::<String>());
        world.lines()
            .iter()
            .for_each(|x| {
                x
                .iter()
                .for_each(|x| {
                        print!("{}", x);
                    });
                print!("\n");
            });
        world.reset_position();
        if !removed {
            break;
        }
    }

    println!("Password should be: {}", value);
}
