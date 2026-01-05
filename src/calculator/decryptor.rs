use crate::calculator::{Operator, char_to_num};

pub const LEN_DIGITS: usize = 4;

pub fn decrypt_word(input: &str) -> Result<u32, DecryptError> {
    if input.len() != LEN_DIGITS {
        return Err(DecryptError::InvalidLen);
    }

    let mut numbers = [0; LEN_DIGITS];
    for (index, c) in input.char_indices().take(numbers.len()) {
        let n = char_to_num(c).ok_or(DecryptError::InvalidChar)?;
        numbers[index] = n;
    }

    decrypt_numbers(numbers)
}

pub fn decrypt_numbers(numbers: [u32; LEN_DIGITS]) -> Result<u32, DecryptError> {
    decrypt_recursive(numbers[0], &numbers[1..], Operator::all()).ok_or(DecryptError::NoSolution)
}

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum DecryptError {
    #[display("Invalid input, expected 4 characters")]
    InvalidLen,
    #[display("Invalid input, expected alphabetic character only")]
    InvalidChar,
    #[display("No solution found")]
    NoSolution,
}

fn decrypt_recursive(acc: u32, numbers: &[u32], ops: Operator) -> Option<u32> {
    let b = numbers.first()?;
    ops.iter()
        .filter_map(|op| {
            op.apply(acc, *b).and_then(|acc| {
                let remain = &numbers[1..];
                if remain.is_empty() {
                    Some(acc)
                } else {
                    decrypt_recursive(acc, &numbers[1..], ops.difference(op))
                }
            })
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
