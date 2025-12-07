use std::fmt::Display;

pub enum CharToken {
    Paper,
    Else
}

impl Display for CharToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Paper => "@",
                Self::Else => "."
            }
        )
    }
}

pub type CharLine = Vec<CharToken>;

pub type Lines = Vec<CharLine>;

pub struct WorldParser<'a> {
    content: std::str::Lines<'a>
}

impl<'a> WorldParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            content: input.lines(),
        }
    }

    pub fn tokenize_input(&mut self) -> Lines {
        let mut tokens = Vec::new();

        for line in self.content.clone() {
            let mut chartokens = Vec::new();
            for char in line.chars() {
                match char {
                    char if char == '@' => {
                        chartokens.push(CharToken::Paper);
                    }
                    _ => {
                        chartokens.push(CharToken::Else);
                    }
                }
            }
            tokens.push(chartokens);
        }

        return tokens;
    }
}
