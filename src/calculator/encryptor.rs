use rayon::prelude::*;

use crate::calculator::{ALPHABET, CORE_LENGTH, Letter, decryptor::decrypt_numbers};

/**
   Brute force encryption
   Tries every posible 4-letter combination that reaches the input character
   `input` may be uppercase or lowercase
*/
pub fn encrypt_letter(letter: Letter) -> Vec<[Letter; CORE_LENGTH]> {
    let all_combinations = ALPHABET.flat_map(move |a| {
        ALPHABET.flat_map(move |b| ALPHABET.flat_map(move |c| ALPHABET.map(move |d| [a, b, c, d])))
    });

    let mut output = all_combinations
        .par_bridge()
        .filter_map(|numbers| {
            let core = decrypt_numbers(numbers).ok()?;
            (core == letter.to_num()).then_some(numbers)
        })
        .filter_map(|[a, b, c, d]| {
            Some([
                Letter::try_from_num(a)?,
                Letter::try_from_num(b)?,
                Letter::try_from_num(c)?,
                Letter::try_from_num(d)?,
            ])
        })
        .collect::<Vec<_>>();

    output.sort();

    output
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

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
        for word in known_L {
            let word = word
                .into_iter()
                .filter_map(|c| Letter::try_from(c).ok())
                .collect_array()
                .unwrap();
            assert!(values.contains(&word))
        }
    }
}
