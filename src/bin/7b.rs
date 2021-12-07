use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/7.txt")?;
    let crabs: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let fuel: Option<i64> = crabs
        .iter()
        .map(|&x0| {
            crabs
                .iter()
                .map(|&x1| {
                    let steps = (x1 - x0).abs();
                    steps * (steps + 1) / 2
                })
                .sum()
        })
        .min();
    println!("{:?}", fuel.unwrap());
    Ok(())
}
