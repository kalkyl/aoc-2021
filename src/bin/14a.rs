use std::{collections::HashMap, fs::read_to_string, io::Error};
const NUM_STEPS: u8 = 10;

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/14.txt")?;
    let (initial, rules) = input.split_once("\n\n").unwrap();
    let initial: Vec<_> = initial.chars().collect();
    let initial_pairs: Vec<_> = initial.windows(2).collect();

    let initial_counts = initial_pairs.iter().fold(HashMap::new(), |mut counts, x| {
        *counts.entry((x[0], x[1])).or_insert(0) += 1_u64;
        counts
    });

    let rules: HashMap<_, _> = rules
        .split('\n')
        .map(|l| {
            let (key, ins) = l.split_once(" -> ").unwrap();
            let key: Vec<_> = key.chars().collect();
            ((key[0], key[1]), ins.chars().next().unwrap())
        })
        .collect();

    let pairs = (0..NUM_STEPS).fold(initial_counts, |counts, _| {
        counts
            .iter()
            .fold(HashMap::new(), |mut acc, (&(a, b), cnt)| {
                let ins = *rules.get(&(a, b)).unwrap();
                *acc.entry((a, ins)).or_insert(0) += cnt;
                *acc.entry((ins, b)).or_insert(0) += cnt;
                acc
            })
    });

    let mut final_counts = pairs
        .iter()
        .fold(HashMap::new(), |mut counts, (&(a, _), cnt)| {
            *counts.entry(a).or_insert(0) += cnt;
            counts
        });
    *final_counts.entry(initial[initial.len() - 1]).or_insert(0) += 1;

    let diff = final_counts.values().max().unwrap() - final_counts.values().min().unwrap();
    println!("{}", diff);
    Ok(())
}
