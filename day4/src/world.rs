use std::fmt::Display;

use crate::parser::{CharToken, Lines};

pub struct World {
    lines: Lines,
    position_x: usize,
    position_y: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.lines.iter().map(
                |x| x
                    .iter()
                    .map(
                        |y| format!("{}", y)
                    ).collect::<String>()
            ).collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

impl World {
    pub fn display(&self) {
        println!("{}", self);
    }

    pub fn new(input: Lines) -> Self {
        Self {
            lines: input,
            position_x: 0,
            position_y: 0,
        }
    }

    pub fn should_end(&self) -> bool {
        let max_x = self.lines.first().unwrap_or(&vec![]).len();
        let max_y = self.lines.len();
        if max_y == 0 || max_x == 0 || (self.position_x == max_x - 1 && self.position_y == max_y - 1) {
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) {
        let mut max_x = self.lines.first().unwrap_or(&vec![]).len();
        let mut max_y = self.lines.len();
        if max_y == 0 || max_x == 0 {
            return;
        }
        max_y -= 1;
        max_x -= 1;
        if self.position_x == max_x {
            if self.position_y == max_y {
                panic!("World overflow!");
            }
            self.position_y += 1;
            self.position_x = 0;
        } else {
            self.position_x += 1;
        }
    }

    pub fn current_is_paper(&self) -> bool {
        let max_x = self.lines.first().unwrap_or(&vec![]).len();
        if self.lines.len() == 0 || max_x == 0 {
            return false;
        }
        match self.lines[self.position_y][self.position_x] {
            CharToken::Paper => true,
            CharToken::Else => false,
        }
    }

    pub fn next_paper(&mut self) {
        while !self.current_is_paper() {
            self.next();
            if self.should_end() {
                break;
            }
        }
    }

    pub fn get_papers_around_position(&self) -> u8 {
        let directions = vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest
        ];

        let mut papers = 0;
        for dir in directions {
            if let true = self.is_paper(dir) {
                papers += 1;
            }
        }
        return papers;
    }

    pub fn is_paper(&self, direction: Direction) -> bool {
        let mut max_x = self.lines.first().unwrap_or(&vec![]).len();
        let mut max_y = self.lines.len();
        if max_y == 0 || max_x == 0 {
            return false;
        }
        max_y -= 1;
        max_x -= 1;
        match direction {
            Direction::North => {
                if self.position_y == 0 {
                    return false;
                }
                match self.lines[self.position_y - 1][self.position_x] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::NorthEast => {
                if self.position_y == 0 || self.position_x == max_x {
                    return false;
                }
                match self.lines[self.position_y - 1][self.position_x + 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::East => {
                if self.position_x == max_x {
                    return false;
                }
                match self.lines[self.position_y][self.position_x + 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::SouthEast => {
                if self.position_y == max_y || self.position_x == max_x {
                    return false;
                }
                match self.lines[self.position_y + 1][self.position_x + 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::South => {
                if self.position_y == max_y {
                    return false;
                }
                match self.lines[self.position_y + 1][self.position_x] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::SouthWest => {
                if self.position_y == max_y || self.position_x == 0 {
                    return false;
                }
                match self.lines[self.position_y + 1][self.position_x - 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::West => {
                if self.position_x == 0 {
                    return false;
                }
                match self.lines[self.position_y][self.position_x - 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            },
            Direction::NorthWest => {
                if self.position_y == 0 || self.position_x == 0 {
                    return false;
                }
                match self.lines[self.position_y - 1][self.position_x - 1] {
                    CharToken::Paper => true,
                    CharToken::Else => false,
                }
            }
        }
    }
}
