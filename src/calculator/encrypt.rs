use crate::calculator::{Cypher, Operator, char_to_num, num_to_char};

pub fn encrypt_number(number: u32) -> Result<Vec<Cypher>, String> {
    Err("unimplemented".to_string())
}

pub fn encrypt_letter(input: &str) -> Result<Vec<Cypher>, String> {
    let Some(c) = input.chars().next() else {
        return Err("Invalid input : expected at least one char".to_string());
    };

    if input.len() != 1 {
        return Err("Invalid input : expected a single character".to_string());
    }

    let Some(target) = char_to_num(c) else {
        return Err("Invalid input : expected an alphabetic letter".to_string());
    };

    let mut results = (1..=26_u32)
        .map(|a| {
            (1..=26_u32).flat_map(move |b| {
                Operator::all().into_iter().flat_map(move |op| {
                    op.apply(a, b)
                        .map(|b1| ([a, b], b1, Operator::all().difference(op)))
                })
            })
        })
        .flatten()
        .map(move |([a, b], b1, op_remain)| {
            (1..=26_u32).flat_map(move |c| {
                op_remain.iter().flat_map(move |op| {
                    op.apply(b1, c)
                        .map(|c1| ([a, b, c], c1, op_remain.difference(op)))
                })
            })
        })
        .flatten()
        .map(|([a, b, c], c1, op_remain)| {
            (1..=26_u32).flat_map(move |d| {
                op_remain.iter().flat_map(move |op| {
                    op.apply(c1, d).and_then(|d1| {
                        (d1 == target).then_some({
                            let a = num_to_char(a);
                            let b = num_to_char(b);
                            let c = num_to_char(c);
                            let d = num_to_char(d);
                            Cypher::Text([a, b, c, d])
                        })
                    })
                })
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    results.dedup();

    Ok(results)
}
