pub mod decryptor;
pub mod encryptor;

/// Core is composed of 4 numbers
pub const CORE_LENGTH: usize = 4;

use std::{
    ops::{self, RangeInclusive},
    str::FromStr,
};

use bitflags::bitflags;

bitflags! {
    /**
        Remaining available operations

        The first operation is always an addition,
        the 3 remaining operation (substraction, multiply and division) are implemented as a bitflag.

        Each flag represents a possible operation that remains to be tested
     */
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Operation: u8 {
        const SUB = 1;
        const MUL = 1 << 1;
        const DIV = 1 << 2;
    }
}
impl Operation {
    /**
        Applies the operation `a OP b`

        `self` if must contain only one operation.
        The caller is expected to loop over the available operations using `Op::iter()` and only call `apply()` on each individual operation

        Returns `Some(value)` if the operation is mathematically valid :

        | Op    | Validity     |
        | ----- | :--------    |
        | `MUL` | Always valid |
        | `DIV` | Result must be a whole number |
        | `SUB` | Result must be a positive number |

    */
    fn apply(&self, a: u32, b: u32) -> Option<u32> {
        match *self {
            Self::SUB => a.checked_sub(b),
            Self::MUL => Some(a * b),
            Self::DIV => {
                if a.checked_rem(b) != Some(0) {
                    return None;
                }
                a.checked_div(b)
            }
            _ => unreachable!("Unknown operation: bitflag {self:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Letter(u32);
impl TryFrom<char> for Letter {
    type Error = ParseLetterError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        char_to_num(c).map(Letter).ok_or(ParseLetterError)
    }
}
impl FromStr for Letter {
    type Err = ParseLetterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let Some(c) = chars.next() else {
            return Err(ParseLetterError);
        };
        if chars.next().is_some() {
            return Err(ParseLetterError);
        };
        Letter::try_from(c)
    }
}
impl ops::Deref for Letter {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("Invalid input: expected an alphabetic letter")]
pub struct ParseLetterError;

/// Every letter in the range `'A'..='Z'` converted to cypher numbers
const ALPHABET: RangeInclusive<u32> = 1..=26;

/**
    Converts user provided `char` to cyphered number in the range `1..=26`

    Input expects ASCII letter.
    Uppercase and lowercase are converted to the same number
*/
pub fn char_to_num(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(1 + c as u32 - 'a' as u32),
        'A'..='Z' => Some(1 + c as u32 - 'A' as u32),
        _ => None,
    }
}

/**
    Converts cypher number in the range `1..=26` to char

    Returns None if the character is invalid
*/
pub fn num_to_char(num: u32) -> Option<char> {
    ALPHABET
        .contains(&num)
        .then(|| unsafe { char::from_u32_unchecked('A' as u32 - 1 + num) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet() {
        for (num, c) in ALPHABET.zip('A'..='Z') {
            assert_eq!(num_to_char(num), Some(c));
            assert_eq!(char_to_num(c), Some(num));
        }
    }

    #[test]
    fn from_char_to_num() {
        assert_eq!(char_to_num('A'), Some(1));
        assert_eq!(char_to_num('a'), Some(1));
        assert_eq!(char_to_num('Z'), Some(26));
        assert_eq!(char_to_num('z'), Some(26));
    }

    #[test]
    fn from_num_to_char() {
        assert_eq!(num_to_char(1), Some('A'));
    }
}
