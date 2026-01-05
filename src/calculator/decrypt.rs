use crate::calculator::{Operator, char_to_num};

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
