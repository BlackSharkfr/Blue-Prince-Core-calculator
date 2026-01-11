use std::str::FromStr;

use itertools::Itertools;

use crate::calculator::{CORE_LENGTH, Letter, Operation};

/**
    Computes the numeric core from the input 4-letter `word`

    Input `word` must be a 4 alphabetic character string.
    Both uppercase and lowercase are allowed and produce the same result

    # Errors
    - Invalid input
    - No solution found
*/
pub fn decrypt_word(word: &str) -> Result<u32, DecryptError> {
    if word.len() != CORE_LENGTH {
        return Err(DecryptError::InputWordLen);
    }

    let numbers = word
        .chars()
        .flat_map(Letter::try_from)
        .map(Letter::to_num)
        .collect_array()
        .ok_or(DecryptError::InputLetter)?;

    decrypt_numbers(numbers)
}

/**
    Computes the numeric core from the 4 input `numbers`

    # Errors
    - No solution found
*/
pub fn decrypt_numbers(numbers: [u32; CORE_LENGTH]) -> Result<u32, DecryptError> {
    let first = numbers[0];
    let remaining_3 = &numbers[1..];
    decrypt_recursive(first, remaining_3, Operation::all()).ok_or(DecryptError::NoSolution)
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, derive_more::Error)]
pub enum DecryptError {
    #[display("Invalid characters, expected alphabetic words or numbers")]
    InputEmpty,
    #[display("Invalid length, expected 4 character words")]
    InputWordLen,
    #[display("Invalid length, expected 4 distinct numbers")]
    InputNumsLen,
    #[display("Invalid character, expected alphabetic character")]
    InputLetter,
    #[display("Found words mixed with numbers")]
    InputMixed,
    #[display("No solution found")]
    NoSolution,
}

#[derive(Debug, Clone)]
pub enum DecryptInput {
    Words(Vec<String>),
    Numbers([u32; CORE_LENGTH]),
}
impl FromStr for DecryptInput {
    type Err = DecryptError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut is_digits = false;
        let mut is_alphabetic = false;
        let words = input
            .split_whitespace()
            .map(|str| {
                for c in str.chars() {
                    if c.is_ascii_digit() {
                        is_digits = true;
                    } else if c.is_ascii_alphabetic() {
                        is_alphabetic = true;
                    }
                }
                str.to_string()
            })
            .collect::<Vec<_>>();

        match (is_digits, is_alphabetic) {
            (true, false) => {
                if words.len() != CORE_LENGTH {
                    return Err(DecryptError::InputNumsLen);
                }
                words
                    .into_iter()
                    .filter_map(|word| word.parse::<u32>().ok())
                    .collect_array()
                    .map(DecryptInput::Numbers)
                    .ok_or(DecryptError::InputNumsLen)
            }
            (false, true) => Ok(DecryptInput::Words(words)),
            (false, false) => Err(DecryptError::InputEmpty),
            (true, true) => Err(DecryptError::InputMixed),
        }
    }
}

/**
    Brute force decryption

    Pulls the first remaining number and tries to apply it using every possible remaining `Operator`
*/
fn decrypt_recursive(acc: u32, numbers: &[u32], ops: Operation) -> Option<u32> {
    let Some((first, remain)) = numbers.split_first() else {
        return Some(acc);
    };
    ops.iter()
        .filter_map(|op| {
            op.apply(acc, *first)
                .and_then(|total| decrypt_recursive(total, remain, ops.difference(op)))
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_letters() {
        assert_eq!(decrypt_word("PEAK").unwrap(), Letter('A').to_num());
        assert_eq!(decrypt_word("TREE").unwrap(), Letter('B').to_num());
        assert_eq!(decrypt_word("JOYA").unwrap(), Letter('E').to_num());
        assert_eq!(decrypt_word("MAIL").unwrap(), Letter('I').to_num());
        assert_eq!(decrypt_word("ROCK").unwrap(), Letter('K').to_num());
        assert_eq!(decrypt_word("DATE").unwrap(), Letter('L').to_num());
        assert_eq!(decrypt_word("WILL").unwrap(), Letter('N').to_num());
        assert_eq!(decrypt_word("VASE").unwrap(), Letter('O').to_num());
        assert_eq!(decrypt_word("WELL").unwrap(), Letter('R').to_num());
        assert_eq!(decrypt_word("PIGS").unwrap(), Letter('S').to_num());
        assert_eq!(decrypt_word("SAND").unwrap(), Letter('T').to_num());
        assert_eq!(decrypt_word("CLAM").unwrap(), Letter('W').to_num());
    }

    #[test]
    fn known_numbers() {
        assert_eq!(decrypt_numbers([1000, 200, 11, 2]), Ok(53))
    }
}
