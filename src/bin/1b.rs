use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.map(|v| v.parse().unwrap()))
        .collect::<Result<Vec<u32>, _>>()?;
    let n = entries
        .windows(4)
        .filter(|w| w.iter().take(3).sum::<u32>() < w.iter().skip(1).sum())
        .count();
    println!("{}", n);
    Ok(())
}
