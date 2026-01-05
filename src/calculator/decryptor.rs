use crate::calculator::{Operator, char_to_num};

pub fn decrypt(input: &str) -> (Vec<Option<u32>>, Vec<String>) {
    let cyphers = match input
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c.is_whitespace())
    {
        false => {
            // Single numeric core
            vec![input]
        }
        true => {
            // Cyphertext
            input.split_ascii_whitespace().collect()
        }
    };

    let mut cores = Vec::new();
    let mut errors = Vec::new();
    for cypher in cyphers {
        let Some(numbers) = parse_input(cypher) else {
            errors.push("Invalid input".to_string());
            cores.push(None);
            continue;
        };
        match solve_recursive(numbers[0], &numbers[1..], Operator::all()) {
            Some(core) => cores.push(Some(core)),
            None => {
                errors.push(format!("No solutions for {cypher:?}"));
                cores.push(None)
            }
        }
    }
    (cores, errors)
}

fn parse_input(str: &str) -> Option<[u32; 4]> {
    let words = str.split_whitespace().collect::<Vec<_>>();
    if words.len() == 4 {
        let mut nums = [0; 4];
        for (index, num) in nums.iter_mut().enumerate() {
            *num = words.get(index).and_then(|s| s.parse().ok())?;
        }
        return Some(nums);
    }
    if words.len() != 1 {
        return None;
    }
    let str = *words.first()?;
    if str.len() != 4 {
        return None;
    }
    let mut chars = str.chars();
    let mut numbers = [0_u32; 4];
    for num in &mut numbers {
        let c = chars.next()?;
        *num = char_to_num(c)?;
    }
    Some(numbers)
}

fn solve_recursive(acc: u32, numbers: &[u32], ops: Operator) -> Option<u32> {
    let b = numbers.first()?;
    ops.iter()
        .filter_map(|op| {
            op.apply(acc, *b).and_then(|acc| {
                let remain = &numbers[1..];
                if remain.is_empty() {
                    Some(acc)
                } else {
                    solve_recursive(acc, &numbers[1..], ops.difference(op))
                }
            })
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_letter() {
        assert_eq!(decrypt("PEAK"), (vec![Some(1)], Vec::new()));
        assert_eq!(decrypt("TREE"), (vec![Some(2)], Vec::new()));
        assert_eq!(decrypt("JOYA"), (vec![Some(5)], Vec::new()));
        assert_eq!(decrypt("MAIL"), (vec![Some(9)], Vec::new()));
        assert_eq!(decrypt("ROCK"), (vec![Some(11)], Vec::new()));
        assert_eq!(decrypt("DATE"), (vec![Some(12)], Vec::new()));
        assert_eq!(decrypt("WILL"), (vec![Some(14)], Vec::new()));
        assert_eq!(decrypt("VASE"), (vec![Some(15)], Vec::new()));
        assert_eq!(decrypt("WELL"), (vec![Some(18)], Vec::new()));
        assert_eq!(decrypt("PIGS"), (vec![Some(19)], Vec::new()));
        assert_eq!(decrypt("SAND"), (vec![Some(20)], Vec::new()));
        assert_eq!(decrypt("CLAM"), (vec![Some(23)], Vec::new()));
    }

    #[test]
    fn multi_letter() {
        assert_eq!(
            decrypt("PIGS SAND MAIL DATE HEAD"),
            (
                vec![Some(19), Some(20), Some(9), Some(12), Some(12)],
                Vec::new()
            )
        );
    }
}
