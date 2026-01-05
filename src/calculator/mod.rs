pub mod decryptor;
pub mod encryptor;

use std::fmt::Display;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Operator: u8 {
        const SUB = 1;
        const MUL = 1 << 1;
        const DIV = 1 << 2;
    }
}
impl Operator {
    fn apply(&self, a: u32, b: u32) -> Option<u32> {
        match *self {
            Self::SUB => {
                if !self.difference(Self::SUB).is_empty() {
                    panic!("Invalid operator {self:?}")
                }
                a.checked_sub(b)
            }
            Self::MUL => {
                if !self.difference(Self::MUL).is_empty() {
                    panic!("Invalid operator {self:?}")
                }
                Some(a * b)
            }
            Self::DIV => {
                if !self.difference(Self::DIV).is_empty() {
                    panic!("Invalid operator {self:?}")
                }
                if a.checked_rem(b) != Some(0) {
                    return None;
                }
                a.checked_div(b)
            }
            _ => panic!("Invalid operator {self:?}"),
        }
    }
}

pub fn char_to_num(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(1 + c as u32 - 'a' as u32),
        'A'..='Z' => Some(1 + c as u32 - 'A' as u32),
        _ => None,
    }
}

pub fn num_to_char(num: u32) -> char {
    char::from_u32('A' as u32 - 1 + num).unwrap()
}

#[derive(PartialEq)]
pub enum Cypher {
    Numeric([u32; 4]),
    Text([char; 4]),
}
impl Display for Cypher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cypher::Numeric([a, b, c, d]) => write!(f, "{a} {b} {c} {d}"),
            Cypher::Text([a, b, c, d]) => write!(f, "{a}{b}{c}{d}"),
        }
    }
}
