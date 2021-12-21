use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("01.test")?;
    let depths: Vec<i32> = BufReader::new(file)
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(|item| item.parse::<i32>())
        .map(|item| item.unwrap())
        .collect();

    let mut count = 0;
    let mut prev = &depths[0];

    for depth in depths.iter().skip(1) {
        if prev < depth {
            count += 1;
        }
        prev = depth;
    }
    println!("result: {}", count);
    Ok(())
}
