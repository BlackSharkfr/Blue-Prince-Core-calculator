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
    use super::*;

    #[test]
    fn known_letters() {
        assert_eq!(decrypt_word("PEAK"), Ok(1));
        assert_eq!(decrypt_word("TREE"), Ok(2));
        assert_eq!(decrypt_word("JOYA"), Ok(5));
        assert_eq!(decrypt_word("MAIL"), Ok(9));
        assert_eq!(decrypt_word("ROCK"), Ok(11));
        assert_eq!(decrypt_word("DATE"), Ok(12));
        assert_eq!(decrypt_word("WILL"), Ok(14));
        assert_eq!(decrypt_word("VASE"), Ok(15));
        assert_eq!(decrypt_word("WELL"), Ok(18));
        assert_eq!(decrypt_word("PIGS"), Ok(19));
        assert_eq!(decrypt_word("SAND"), Ok(20));
        assert_eq!(decrypt_word("CLAM"), Ok(23));
    }

    #[test]
    fn known_numbers() {
        assert_eq!(decrypt_numbers([1000, 200, 11, 2]), Ok(53))
    }
}
