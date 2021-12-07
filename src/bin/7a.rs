use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/7.txt")?;
    let crabs: Vec<i32> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let fuel: Option<i32> = crabs
        .iter()
        .map(|&x0| crabs.iter().map(|&x1| (x1 - x0).abs()).sum())
        .min();
    println!("{:?}", fuel.unwrap());
    Ok(())
}
    