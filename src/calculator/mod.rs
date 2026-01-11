pub mod decryptor;
pub mod encryptor;

/// Core is composed of 4 numbers
pub const CORE_LENGTH: usize = 4;

use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letter(char);
impl Letter {
    pub fn try_from_char(c: char) -> Option<Self> {
        match c {
            'A'..='Z' => Some(Self(c)),
            'a'..='z' => Some(Self(c.to_ascii_uppercase())),
            _ => None,
        }
    }

    pub fn try_from_num(num: u32) -> Option<Self> {
        ALPHABET
            .contains(&num)
            .then(|| unsafe { char::from_u32_unchecked('A' as u32 - 1 + num) })
            .map(Self)
    }

    pub fn to_num(self) -> u32 {
        1 + self.0 as u32 - 'A' as u32
    }

    pub fn to_char(self) -> char {
        self.into()
    }
}
impl TryFrom<u32> for Letter {
    type Error = ParseLetterError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_num(value).ok_or(ParseLetterError)
    }
}
impl TryFrom<char> for Letter {
    type Error = ParseLetterError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::try_from_char(c).ok_or(ParseLetterError)
    }
}
impl FromStr for Letter {
    type Err = ParseLetterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseLetterError);
        }
        s.chars()
            .next()
            .ok_or(ParseLetterError)
            .and_then(Letter::try_from)
    }
}
impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        value.0
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromIterator<Letter> for String {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        iter.into_iter().map(|letter| letter.0).collect()
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("Invalid input: expected an alphabetic letter")]
pub struct ParseLetterError;

/// Every letter in the range `'A'..='Z'` converted to cypher numbers
const ALPHABET: RangeInclusive<u32> = 1..=26;

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn alphabet() {
        assert_eq!(ALPHABET.count(), 26);
        assert_eq!(ALPHABET.unique().count(), 26);
    }

    #[test]
    fn letter_conversions() {
        let numbers: [Letter; 26] = ALPHABET
            .flat_map(|num| Letter::try_from_num(num))
            .collect_array()
            .unwrap();

        let chars_uppercase: [Letter; 26] = ('A'..='Z')
            .flat_map(|c| Letter::try_from(c))
            .collect_array()
            .unwrap();
        let chars_lowercase: [Letter; 26] = ('a'..='z')
            .flat_map(|c| Letter::try_from(c))
            .collect_array()
            .unwrap();
        let strings: [Letter; 26] = ('A'..='Z')
            .map(|c| c.to_string())
            .flat_map(|str| str.parse::<Letter>())
            .collect_array()
            .unwrap();

        assert_eq!(numbers, strings);
        assert_eq!(chars_uppercase, chars_lowercase);
        assert_eq!(numbers, chars_lowercase);
    }
}
