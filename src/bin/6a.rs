use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/6.txt")?;
    let mut fish: Vec<u8> = input.split(',').map(|x| x.parse().unwrap()).collect();
    for _ in 0..80 {
        let spawn = fish.iter().filter(|&&x| x == 0).count();
        for f in fish.iter_mut() {
            *f = if *f > 0 { *f - 1 } else { 6 };
        }
        fish.extend((0..spawn).map(|_| 8));
    }
    println!("{:?}", fish.iter().count());
    Ok(())
}
