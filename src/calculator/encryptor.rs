use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::calculator::{Operation, char_to_num, num_to_char};

/// Every letter in the range `'A'..='Z'` converted to cypher numbers
const ALPHABET: RangeInclusive<u32> = 1..=26;

/**
   Brute force encryption
   Tries every posible 4-letter combination that reaches the input character
   `input` may be uppercase or lowercase
*/
pub fn encrypt_letter(c: char) -> Result<Vec<[char; 4]>, String> {
    let target = char_to_num(c).ok_or("Invalid input : expected an alphabetic letter")?;

    // The flat_map() and Operator::apply() functions filter out invalid letter combinations early,
    // and prevent the nested for-loops to drift too far to the right
    let results = ALPHABET
        .flat_map(|a| {
            ALPHABET.flat_map(move |b| {
                Operation::all().into_iter().flat_map(move |op| {
                    op.apply(a, b)
                        .map(|total| ([a, b], total, Operation::all().difference(op)))
                })
            })
        })
        .flat_map(move |([a, b], total, op_remain)| {
            ALPHABET.flat_map(move |c| {
                op_remain.iter().flat_map(move |op| {
                    op.apply(total, c)
                        .map(|total| ([a, b, c], total, op_remain.difference(op)))
                })
            })
        })
        .flat_map(|([a, b, c], total, op_remain)| {
            ALPHABET.flat_map(move |d| {
                op_remain.iter().flat_map(move |op| {
                    op.apply(total, d).and_then(|total| {
                        (total == target).then_some({
                            // a,b,c,d are in the range 'A'..='Z', therefore '?' shall never be returned
                            let a = num_to_char(a).unwrap_or('?');
                            let b = num_to_char(b).unwrap_or('?');
                            let c = num_to_char(c).unwrap_or('?');
                            let d = num_to_char(d).unwrap_or('?');
                            [a, b, c, d]
                        })
                    })
                })
            })
        })
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    // Deduplication is required because some duplicates exist when intermediate calculations cancel each other
    // Example :
    //      a * 1 / 1 - d
    //      a / 1 * 1 - d

    Ok(results)
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
    }
}
