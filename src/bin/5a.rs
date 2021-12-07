use std::{collections::HashMap, fs::read_to_string, io::Error};

fn line_from_str(str: &str) -> ((usize, usize), (usize, usize)) {
    let mut coords: Vec<_> = str
        .split(" -> ")
        .map(|row| {
            match row
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                [x, y] => (x, y),
                _ => unreachable!(),
            }
        })
        .collect();
    coords.sort_by_key(|(x, _)| *x);
    (coords[0], coords[1])
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/5.txt")?;
    let lines: Vec<_> = input.split("\n").map(line_from_str).collect();
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for ((x1, y1), (x2, y2)) in lines
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
    {
        for y in (*y1.min(y2))..=(*y2.max(y1)) {
            for x in *x1..=*x2 {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    let n = map.values().filter(|&&c| c > 1).count();
    println!("{:?}", n);
    Ok(())
}
