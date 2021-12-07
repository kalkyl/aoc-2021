use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/6.txt")?;
    let fish: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let mut age_counts = [0_usize; 9];
    for f in fish {
        age_counts[f] += 1;
    }
    for _ in 0..256 {
        age_counts.rotate_left(1);
        age_counts[6] += age_counts[8];
    }
    println!("{:?}", age_counts.iter().sum::<usize>());
    Ok(())
}
