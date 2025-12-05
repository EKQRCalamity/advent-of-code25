// Rules are if rotation would go < 0 go to 99, if its above 99 go to 0

use crate::parser::RotationToken;

pub struct Rotator {
    rotation_value: u16
}

impl Rotator {
    pub fn new(initial: u16) -> Self {
        Self {
            rotation_value: initial,
        }
    }

    pub fn rotate(&mut self, token: &RotationToken) -> u16 {
        match token {
            RotationToken::Left(x) => {
                let mut x = *x;
                loop {
                    if x == 0 { break; }
                    self.rotate_left();
                    x -= 1;
                }
            },
            RotationToken::Right(x) => {
                let mut x = *x;
                loop {
                    if x == 0 { break; }
                    self.rotate_right();
                    x -= 1;
                }
            },
            RotationToken::Malformed(x) => {
                eprintln!("{}", x);
            }
        }
        self.rotation_value
    }

    fn rotate_left(&mut self) -> u16 {
        if self.rotation_value == 0 {
            self.rotation_value = 99;
        } else {
            self.rotation_value -= 1;
        }
        self.rotation_value
    }

    fn rotate_right(&mut self) -> u16 {
        if self.rotation_value == 99 {
            self.rotation_value = 0;
        } else {
            self.rotation_value += 1;
        }
        self.rotation_value
    }
}
