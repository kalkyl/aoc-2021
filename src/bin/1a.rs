use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.map(|v| v.parse().unwrap()))
        .collect::<Result<Vec<u32>, _>>()?;
    let n = entries.windows(2).filter(|w| w[1] > w[0]).count();
    println!("{}", n);
    Ok(())
}
