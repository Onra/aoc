use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn is_larger(m1: [&i32; 3], m2: [&i32; 3]) -> bool {
    let a: i32 = m1.into_iter().sum();
    let b: i32 = m2.into_iter().sum();

    return a > b;
}

fn main() -> Result<()> {
    let file = File::open("01.input")?;
    let depths: Vec<i32> = BufReader::new(file)
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .map(|item| item.parse::<i32>())
        .map(|item| item.unwrap())
        .collect();

    let mut count = 0;
    let mut prev: [&i32; 3] = [&depths[0], &depths[1], &depths[2]];
    let mut curr: [&i32; 3] = [&depths[1], &depths[2], &depths[3]];
    let mut idx = 3;

    for c in depths.iter().skip(4) {
        if is_larger(curr, prev) {
            count += 1;
        }

        prev[idx % 3] = curr[(idx - 1) % 3];
        curr[idx % 3] = c;

        idx += 1;
    }

    if is_larger(curr, prev) {
        count += 1;
    }

    println!("result: {}", count);
    Ok(())
}
