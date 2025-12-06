use std::fmt::Display;

pub enum BankToken {
    Battery(String),
    Malformed(String)
}

impl Display for BankToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Battery(x) => x.to_string(),
                Self::Malformed(x) => x.to_string(),
            }
        )
    }
}

pub struct BankParser<'a> {
    content: std::str::Lines<'a>,
}

impl<'a> BankParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            content: input.lines(),
        }
    }

    pub fn tokenize_input(&mut self) -> Vec<BankToken> {
        let mut tokens = Vec::new();

        for line in self.content.clone() {
            match line.chars().take(5).collect::<String>().parse::<u64>() {
                Ok(_x) => tokens.push(BankToken::Battery(line.to_string())),
                Err(e) => tokens.push(BankToken::Malformed(e.to_string())),
            }
        }

        return tokens;
    }
}
