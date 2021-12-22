use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("02.input")?;
    let directions: Vec<String> = BufReader::new(file)
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    let mut depth = 0;
    let mut h_position = 0;

    for direction in directions.iter() {
        let split_iter: Vec<&str> = direction.split_whitespace().collect();
        let dir = split_iter[0].to_string();
        let movement: i32 = split_iter[1].to_string().parse().unwrap();

        if dir == "forward" {
            h_position += movement;
        } else if dir == "up" {
            depth -= movement;
        } else {
            depth += movement;
        }
    }

    println!("Response: {}", depth * h_position);

    Ok(())
}
