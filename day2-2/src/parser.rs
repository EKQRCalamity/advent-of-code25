use std::fmt::Display;

pub enum RangeToken {
    Range(u64, u64),
    InvalidRange(u64, u64),
    Malformed(String)
}

impl Display for RangeToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Range(lower, upper) => format!("R {}->{}", lower, upper),
                Self::InvalidRange(upper, lower) => format!("Invalid Range, can't shrink the range! {}->{}", upper, lower),
                Self::Malformed(x) => format!("Malformed token: {}", x),
            }
        )
    }
}

pub struct RangeParser<'a> {
    content: std::str::Split<'a, &'a str>,
}

impl<'a> RangeParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            content: input.split(",")
        }
    }

    pub fn replace_input(&mut self, input: &'a str) {
        self.content = input.split(",");
    }

    pub fn tokenize_input(&mut self) -> Vec<RangeToken> {
        let mut tokens = Vec::new();

        for range in self.content.clone() {
            match range {
                range if range.contains("-") => {
                    let mut nums = range.splitn(2, "-");
                    let x: &'a str = nums.next().expect("First range value not present!").trim();
                    let y: &'a str = nums.next().expect("Second range value not present!").trim();

                    println!("X: {} Y: {}", x, y);

                    let xnum = x.parse::<u64>().expect("Invalid Range in input (x)");
                    let ynum = y.parse::<u64>().expect("Invalid Range in input (y)");

                    if xnum > ynum {
                        tokens.push(RangeToken::InvalidRange(xnum, ynum));
                    } else {
                        tokens.push(RangeToken::Range(xnum, ynum));
                    }
                }
                _ => tokens.push(RangeToken::Malformed(format!("Range '{}' is not in correct format!", range))),
            }
        }

        return tokens;
    }
}
