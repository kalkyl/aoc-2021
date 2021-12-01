use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.map(|v| v.parse().unwrap()))
        .collect::<Result<Vec<u32>, _>>()?;
    let (n, _) = entries.iter().fold((0_u32, None), |(n, prev), &curr| {
        match prev.map(|p| curr > p) {
            Some(true) => (n + 1, Some(curr)),
            _ => (n, Some(curr)),
        }
    });
    println!("{}", n);
    Ok(())
}
