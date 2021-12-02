use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/2.txt")?)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    let (x, y) = directions.iter().fold((0_u32, 0_u32), |(x, y), line| {
        match line.split(' ').collect::<Vec<_>>().as_slice() {
            ["up", d] => (x, y - d.parse::<u32>().unwrap()),
            ["down", d] => (x, y + d.parse::<u32>().unwrap()),
            ["forward", d] => (x + d.parse::<u32>().unwrap(), y),
            _ => (x, y),
        }
    });
    println!("{}", x * y);
    Ok(())
}
