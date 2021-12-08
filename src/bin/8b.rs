use std::{fs::read_to_string, io::Error};
const DIGITS: [&'static str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[derive(Debug)]
struct Entry<'a> {
    patterns: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a> Entry<'a> {
    fn from_str(str: &'a str) -> Self {
        let (patterns, output) = str.split_once(" | ").unwrap();
        Self {
            patterns: patterns.split_whitespace().collect(),
            output: output.split_whitespace().collect(),
        }
    }
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/8.txt")?;
    let entries: Vec<_> = input.split('\n').map(Entry::from_str).collect();
    let sum = entries
        .iter()
        .map(|entry| {
            let digits = DIGITS.map(|d| {
                entry
                    .patterns
                    .iter()
                    .find(|p| p.chars().count() == d.chars().count())
                    .unwrap()
            });
            entry
                .output
                .iter()
                .map(|out| {
                    match out.chars().count() {
                        5 => {
                            if digits[1].chars().all(|d| out.chars().any(|c| c == d)) {
                                3
                            } else if digits[8]
                                .chars()
                                .filter(|c8| !digits[4].chars().any(|c4| c4 == *c8))
                                .all(|d| out.chars().any(|c| c == d))
                            {
                                2
                            } else {
                                5
                            }
                        }
                        6 => {
                            if digits[4].chars().all(|d| out.chars().any(|c| c == d)) {
                                9
                            } else if digits[7].chars().all(|d| out.chars().any(|c| c == d)) {
                                0
                            } else {
                                6
                            }
                        }
                        n => digits.iter().position(|s| s.chars().count() == n).unwrap(),
                    }
                    .to_string()
                })
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>();
    println!("{:?}", sum);
    Ok(())
}
