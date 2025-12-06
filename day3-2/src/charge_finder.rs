use crate::parser::BankToken;

pub struct Finder {}

fn highest_possible_number(value: &String) -> u128 {
    let digits: Vec<u8> = value
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let n = digits.len();

    let mut result = Vec::with_capacity(12);
    let mut start = 0;

    for remaining in (0..12).rev() {
        let end = n - remaining;

        let mut best_digit = 0;
        let mut best_index = start;

        for i in start..end {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_index = i;

                if best_digit == 9 {
                    break;
                }
            }
        }

        result.push(best_digit);
        start = best_index + 1;
    }

    let mut number: u128 = 0;
    for digit in result {
        number = number * 10 + digit as u128;
    }

    number
}

impl Finder {
    pub fn find(token: &BankToken) -> u128 {
        return match token {
            BankToken::Battery(x) => highest_possible_number(x),
            BankToken::Malformed(e) => {
                eprintln!("Malformed token: {}", e);
                0
            }
        };
    }

    pub fn find_all(tokens: Vec<&BankToken>) -> Vec<u128> {
        tokens.iter().map(|x| Self::find(x)).collect::<Vec<u128>>()
    }
}
