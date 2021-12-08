use std::{fs::read_to_string, io::Error};
const DIGITS: [&'static str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn output_from_str(str: &str) -> Vec<&str> {
    let (_, output) = str.split_once(" | ").unwrap();
    output.split_whitespace().collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/8.txt")?;
    let outputs: Vec<_> = input.split('\n').map(output_from_str).collect();
    let lengths = [1, 4, 7, 8].map(|n| DIGITS[n].chars().count());
    let n = outputs
        .iter()
        .flatten()
        .filter(|d| lengths.iter().any(|&l| l == d.chars().count()))
        .count();
    println!("{:?}", n);
    Ok(())
}
