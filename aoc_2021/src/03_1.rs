use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn get_decimal_from_binary(binary_value: &str) -> isize {
    return isize::from_str_radix(binary_value, 2).unwrap();
}

fn main() -> Result<()> {
    let file = File::open("03.input")?;
    let reports: Vec<String> = BufReader::new(file)
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    let mut occurrences: [(i32, i32); 12] = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for report in reports.iter() {
        let report_chars: Vec<char> = report.chars().collect();

        for idx in 0..12 {
            if report_chars[idx] == '0' {
                occurrences[idx].0 += 1;
            } else {
                occurrences[idx].1 += 1;
            }
        }
    }

    let mut gamma_rate = "".to_string();
    let mut epsilon_rate = "".to_string();
    println!("occurences: {:?}", occurrences);

    for occurence in occurrences.iter() {
        if occurence.0 > occurence.1 {
            gamma_rate += "0";
            epsilon_rate += "1";
        } else {
            gamma_rate += "1";
            epsilon_rate += "0";
        }
    }

    println!("gamma_rate {}", gamma_rate);
    println!("epsilon_rate {}", epsilon_rate);

    let gamma_decimal = get_decimal_from_binary(&gamma_rate);
    let epsilon_decimal = get_decimal_from_binary(&epsilon_rate);

    println!("gamma_decimal {}", gamma_decimal);
    println!("epsilon_decimal {}", epsilon_decimal);

    println!(
        "Response: {}",
        get_decimal_from_binary(&gamma_rate) * get_decimal_from_binary(&epsilon_rate)
    );

    Ok(())
}
