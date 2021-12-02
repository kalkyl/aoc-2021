use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let directions = BufReader::new(File::open("./input/2.txt")?)
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    let (_, x, y) = directions
        .iter()
        .fold((0u32, 0_u32, 0_u32), |(aim, x, y), line| {
            match line.split(' ').collect::<Vec<_>>().as_slice() {
                ["up", d] => (aim - d.parse::<u32>().unwrap(), x, y),
                ["down", d] => (aim + d.parse::<u32>().unwrap(), x, y),
                ["forward", d] => {
                    let d = d.parse::<u32>().unwrap();
                    (aim, x + d, y + aim * d)
                }
                _ => (aim, x, y),
            }
        });
    println!("{}", x * y);
    Ok(())
}
