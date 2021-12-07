use std::{collections::HashMap, fs::read_to_string, io::Error};

fn board_from_str(str: &&str) -> HashMap<u8, (usize, usize, bool)> {
    str.split('\n')
        .enumerate()
        .map(|(y, r)| {
            r.split_whitespace()
                .enumerate()
                .map(|(x, c)| (c.parse::<u8>().unwrap(), (x, y, false)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/4.txt")?;
    let input: Vec<_> = input.split("\n\n").collect();
    let calls: Vec<u8> = input[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut boards: Vec<_> = input.iter().skip(1).map(board_from_str).collect();
    let score = boards
        .iter_mut()
        .filter_map(|b| {
            for (i, call) in calls.iter().enumerate() {
                let v = match b.get_mut(&call) {
                    Some(v) => {
                        *v = (v.0, v.1, true);
                        Some((v.0, v.1))
                    }
                    _ => None,
                };
                if let Some((x, y)) = v {
                    if &b.values().filter(|n| n.0 == x && n.2).count() == &5
                        || &b.values().filter(|n| n.1 == y && n.2).count() == &5
                    {
                        let sum_unmarked: u32 =
                            b.iter().filter(|(_, v)| !v.2).map(|(&n, _)| n as u32).sum();
                        return Some((i, sum_unmarked * *call as u32));
                    }
                }
            }
            None
        })
        .min_by_key(|(calls, _)| *calls)
        .map(|(_, score)| score);
    println!("{:?}", score);
    Ok(())
}
