#[macro_use]
extern crate lazy_static;

mod digital_display;
mod frequency_analysis;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./day_08_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut known_digit_totals = 0;
    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            let output_digits = digital_display::extract_output_digits(&entry.trim());

            known_digit_totals += digital_display::count_known_digits(output_digits);
        }
    }

    println!("{}", known_digit_totals);

    Ok(())
}
