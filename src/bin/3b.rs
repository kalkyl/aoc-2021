use std::fs::File;
use std::io::{BufRead, BufReader, Error};

enum RatingType {
    O,
    CO2,
}

fn rating(vec: &Vec<u16>, rating_type: RatingType) -> Option<u16> {
    let mut list = vec.clone();
    let mut i = 11;
    while list.len() > 1 {
        let (b1, b0): (Vec<u16>, Vec<u16>) = list.iter().partition(|&&x| ((x >> i) & 1) != 0);
        list = list
            .into_iter()
            .filter(|&x| match rating_type {
                RatingType::O => (((x >> i) & 1) != 0) == (b1.len() >= b0.len()),
                RatingType::CO2 => (((x >> i) & 1) != 0) != (b1.len() >= b0.len()),
            })
            .collect();
        i -= 1;
    }
    list.into_iter().next()
}

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/3.txt")?)
        .lines()
        .map(|l| l.map(|v| u16::from_str_radix(&v, 2).unwrap()))
        .collect::<Result<Vec<_>, _>>()?;
    let oxygen = rating(&entries, RatingType::O).unwrap();
    let co2 = rating(&entries, RatingType::CO2).unwrap();
    println!("{}", oxygen as u32 * co2 as u32);
    Ok(())
}
