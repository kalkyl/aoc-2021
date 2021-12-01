use std::io::{BufRead, BufReader, Error, ErrorKind::InvalidData};
use std::{collections::VecDeque, fs::File};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.and_then(|v| v.parse().map_err(|e| Error::new(InvalidData, e))))
        .collect::<Result<Vec<u32>, _>>()?;
    let (n, _) = entries.iter().fold(
        (0_u32, VecDeque::with_capacity(4)),
        |(mut n, mut buf), &val| {
            buf.push_front(val);
            if buf.len() == 4 && buf.iter().take(3).sum::<u32>() > buf.iter().skip(1).sum() {
                n += 1;
            };
            buf.truncate(3);
            (n, buf)
        },
    );
    println!("{}", n);
    Ok(())
}
