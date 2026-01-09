use crate::calculator::{CORE_LENGTH, Operation, char_to_num};

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
        return Err(DecryptError::InputLen);
    }

    let mut numbers = [0; CORE_LENGTH];
    for (index, c) in word.char_indices().take(numbers.len()) {
        let n = char_to_num(c).ok_or(DecryptError::InputChar)?;
        numbers[index] = n;
    }

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

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum DecryptError {
    #[display("Invalid input, expected 4 characters")]
    InputLen,
    #[display("Invalid input, expected alphabetic character only")]
    InputChar,
    #[display("No solution found")]
    NoSolution,
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
    use itertools::Itertools;

    use super::*;

    #[test]
    fn known_letters() {
        let peak = "PEAK"
            .chars()
            .filter_map(char_to_num)
            .collect_array()
            .unwrap();
        assert_eq!(decrypt_numbers(peak).ok(), char_to_num('A'));

        assert_eq!(decrypt_word("PEAK").ok(), char_to_num('A'));
        assert_eq!(decrypt_word("TREE").ok(), char_to_num('B'));
        assert_eq!(decrypt_word("JOYA").ok(), char_to_num('E'));
        assert_eq!(decrypt_word("MAIL").ok(), char_to_num('I'));
        assert_eq!(decrypt_word("ROCK").ok(), char_to_num('K'));
        assert_eq!(decrypt_word("DATE").ok(), char_to_num('L'));
        assert_eq!(decrypt_word("WILL").ok(), char_to_num('N'));
        assert_eq!(decrypt_word("VASE").ok(), char_to_num('O'));
        assert_eq!(decrypt_word("WELL").ok(), char_to_num('R'));
        assert_eq!(decrypt_word("PIGS").ok(), char_to_num('S'));
        assert_eq!(decrypt_word("SAND").ok(), char_to_num('T'));
        assert_eq!(decrypt_word("CLAM").ok(), char_to_num('W'));
    }

    #[test]
    fn known_numbers() {
        assert_eq!(decrypt_numbers([1000, 200, 11, 2]), Ok(53))
    }
}
