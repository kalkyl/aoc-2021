use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn to_u16(slice: &[bool]) -> u16 {
    slice.iter().rev().fold(0, |acc, &b| acc * 2 + b as u16)
}

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .map(|l| l.map(|v| u16::from_str_radix(&v, 2).unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    let bit_vec: Vec<bool> = entries
        .iter()
        .fold(vec![0_usize; 12], |sums, x| {
            sums.iter()
                .enumerate()
                .map(|(i, &s)| s + ((x >> i) & 1) as usize)
                .collect()
        })
        .iter()
        .map(|&s| s > entries.len() / 2)
        .collect();
    let gamma = to_u16(bit_vec.as_slice());
    let epsilon = !gamma & 0b0000_1111_1111_1111;
    println!("{:?}", gamma as u32 * epsilon as u32);
    Ok(())
}
