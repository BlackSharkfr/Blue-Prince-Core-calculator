use std::ops::RangeInclusive;

use rayon::prelude::*;

use crate::calculator::{CORE_LENGTH, char_to_num, decryptor::decrypt_numbers, num_to_char};

/// Every letter in the range `'A'..='Z'` converted to cypher numbers
const ALPHABET: RangeInclusive<u32> = 1..=26;

/**
   Brute force encryption
   Tries every posible 4-letter combination that reaches the input character
   `input` may be uppercase or lowercase
*/
pub fn encrypt_letter(c: char) -> Result<Vec<[char; CORE_LENGTH]>, String> {
    let target = char_to_num(c).ok_or("Invalid input : expected an alphabetic letter")?;

    let all_combinations = ALPHABET.flat_map(move |a| {
        ALPHABET.flat_map(move |b| ALPHABET.flat_map(move |c| ALPHABET.map(move |d| [a, b, c, d])))
    });

    let mut output = all_combinations
        .par_bridge()
        .filter_map(|numbers| {
            let core = decrypt_numbers(numbers).ok()?;
            (core == target).then_some(numbers)
        })
        .filter_map(|[a, b, c, d]| {
            Some([
                num_to_char(a)?,
                num_to_char(b)?,
                num_to_char(c)?,
                num_to_char(d)?,
            ])
        })
        .collect::<Vec<_>>();

    output.sort();

    Ok(output)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn known_letters() {
        let values = encrypt_letter('L').unwrap();
        #[allow(non_snake_case)]
        let known_L = [
            ['D', 'A', 'T', 'E'],
            ['H', 'E', 'A', 'D'],
            ['R', 'O', 'A', 'D'],
        ];
        for word in &known_L {
            assert!(values.contains(word))
        }

        let alternative = encrypt_letter('L').unwrap();
        for word in &known_L {
            assert!(alternative.contains(word));
        }
    }

    #[test]
    fn alphabet() {
        for (num, c) in ALPHABET.zip('A'..='Z') {
            assert_eq!(num_to_char(num), Some(c));
            assert_eq!(char_to_num(c), Some(num));
        }
    }
}
