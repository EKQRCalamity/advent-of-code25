use crate::parser::RangeToken;

pub struct Validator {}

fn repeated_sequence_test(value: u64) -> bool {
    let s = value.to_string();
    let len = s.len();

    for sub_len in 1..=(len / 2) {
        if len % sub_len != 0 {
            continue;
        }

        let seq = &s[..sub_len];
        let times = len / sub_len;
        let mut repeated = String::new();

        for _ in 0..times {
            repeated.push_str(seq);
        }

        if repeated == s {
            return true;
        }
    }

    false
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
                        &repeated_sequence_test
                    ])
            }
        };
    }
}
