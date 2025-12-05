use std::fmt::Display;

pub enum RotationToken {
    Left(u16),
    Right(u16),
    Malformed(String),
}

impl Display for RotationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left(x) => format!("L{}", x),
                Self::Right(x) => format!("R{}", x),
                Self::Malformed(x) => format!("Malformed token: {}", x),
            }
        )
    }
}

pub struct RotationParser<'a> {
    content: std::str::Lines<'a>,
}

impl<'a> RotationParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            content: input.lines(),
        }
    }

    pub fn replace_input(&mut self, input: &'a str) {
        self.content = input.lines();
    }

    pub fn tokenize_input(&mut self) -> Vec<RotationToken> {
        let mut tokens = Vec::new();
        for line in self.content.clone() {
            match line {
                line if line.to_lowercase().starts_with("r") => {
                    let numpart = line.strip_prefix('R').or_else(|| line.strip_prefix('r')).unwrap();
                    match numpart.parse::<u16>() {
                        Ok(x) => tokens.push(RotationToken::Right(x)),
                        Err(e) => tokens.push(RotationToken::Malformed(e.to_string())),
                    }
                }
                line if line.to_lowercase().starts_with("l") => {
                    let numpart = line.strip_prefix('L').or_else(|| line.strip_prefix('l')).unwrap();
                    match numpart.parse::<u16>() {
                        Ok(x) => tokens.push(RotationToken::Left(x)),
                        Err(e) => tokens.push(RotationToken::Malformed(e.to_string())),
                    }
                }
                _ => {
                    tokens.push(RotationToken::Malformed(format!("Line '{}' is not in the correct format.", &line)));
                }
            }
        }
        return tokens;
    }
}
