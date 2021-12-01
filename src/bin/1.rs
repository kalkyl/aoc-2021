use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<u32>, _>>()?;
    let (n, _) = entries.iter().fold((0_u32, None), |(n, prev), &curr| {
        match prev.map(|p: u32| curr > p) {
            Some(true) => (n + 1, Some(curr)),
            _ => (n, Some(curr)),
        }
    });

    println!("{}", n);
    Ok(())
}
