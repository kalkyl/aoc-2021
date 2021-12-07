use std::fs::File;
use std::io::{BufRead, BufReader, Error};

enum RatingType {
    O,
    CO2,
}

fn rating(slice: &[u16], rating_type: RatingType) -> u32 {
    let mut list = slice.to_owned();
    let mut i = 11;
    while list.len() > 1 && i >= 0 {
        let (b1, b0): (Vec<u16>, Vec<u16>) = list.iter().partition(|&&x| ((x >> i) & 1) != 0);
        let mask = b1.len() >= b0.len();
        list = list
            .into_iter()
            .filter(|&x| match rating_type {
                RatingType::O => (((x >> i) & 1) != 0) == mask,
                RatingType::CO2 => (((x >> i) & 1) != 0) != mask,
            })
            .collect();
        i -= 1;
    }
    list.into_iter().next().unwrap() as _
}

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .map(|l| l.map(|v| u16::from_str_radix(&v, 2).unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    let oxygen = rating(&entries, RatingType::O);
    let co2 = rating(&entries, RatingType::CO2);
    println!("{}", oxygen * co2);
    Ok(())
}
