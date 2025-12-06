use crate::parser::BankToken;

pub struct Finder {}

fn highest_possible_number(value: &String) -> u64 {
    let digits: Vec<u32> = value
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut best_fit: u64 = 0;

    for index in 0..digits.len() {
        for next_index in index + 1..digits.len() {
            let num = digits[index] * 10 + digits[next_index];
            if num as u64 > best_fit {
                best_fit = num.into()
            }
        }
    }

    return best_fit;
}

impl Finder {
    pub fn find(token: &BankToken) -> u64 {
        return match token {
            BankToken::Battery(x) => highest_possible_number(x),
            BankToken::Malformed(e) => {
                eprintln!("Malformed token: {}", e);
                0
            }
        };
    }

    pub fn find_all(tokens: Vec<&BankToken>) -> Vec<u64> {
        tokens.iter().map(|x| Self::find(x)).collect::<Vec<u64>>()
    }
}
