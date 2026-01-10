use rayon::prelude::*;

use crate::calculator::{ALPHABET, CORE_LENGTH, Letter, decryptor::decrypt_numbers, num_to_char};

/**
   Brute force encryption
   Tries every posible 4-letter combination that reaches the input character
   `input` may be uppercase or lowercase
*/
pub fn encrypt_letter(letter: Letter) -> Vec<[char; CORE_LENGTH]> {
    let all_combinations = ALPHABET.flat_map(move |a| {
        ALPHABET.flat_map(move |b| ALPHABET.flat_map(move |c| ALPHABET.map(move |d| [a, b, c, d])))
    });

    let mut output = all_combinations
        .par_bridge()
        .filter_map(|numbers| {
            let core = decrypt_numbers(numbers).ok()?;
            (core == *letter).then_some(numbers)
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

    output
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn known_letters() {
        let letter = Letter::try_from('L').unwrap();
        let values = encrypt_letter(letter);
        #[allow(non_snake_case)]
        let known_L = [
            ['D', 'A', 'T', 'E'],
            ['H', 'E', 'A', 'D'],
            ['R', 'O', 'A', 'D'],
        ];
        for word in &known_L {
            assert!(values.contains(word))
        }
    }
}
