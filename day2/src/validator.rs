use crate::parser::RangeToken;

pub struct Validator {}

fn length_test(value: u64) -> bool {
    value.to_string().len() % 2 == 0
}

fn two_repeated_test(value: u64) -> bool {
    let strvalue = value.to_string();

    let mid = strvalue.len() / 2;
    let first = &strvalue[..mid];
    let second = &strvalue[mid..];

    first == second
}

impl Validator {
    // Gives back the invalid ids from the range considering the test functions passed in
    fn validate_range(l: u64, u: u64, tests: &[&dyn Fn(u64) -> bool]) -> Vec<u64> {
        let mut invalids = Vec::new();

        for id in l..=u {
            if tests.iter().all(|test| test(id)) {
                invalids.push(id);
            }
        }

        return invalids;
    }

    pub fn validate_token(token: &RangeToken) -> Vec<u64> {
        return match token {
            RangeToken::InvalidRange(_l, _u) => {
                eprintln!("Lower bound is bigger than upper!");
                Vec::new()
            },
            RangeToken::Malformed(x) => {
                eprintln!("{}", x);
                Vec::new()
            },
            RangeToken::Range(l, u) => {
                Self::validate_range(
                    *l,
                    *u,
                    &[
                        &length_test,
                        &two_repeated_test
                    ])
            }
        };
    }
}
