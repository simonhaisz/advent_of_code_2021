#[macro_use]
extern crate lazy_static;

mod digital_display;
mod frequency_analysis;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::frequency_analysis::FrequencyAnalysis;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_08_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut display_output_total = 0;
    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            let mut analysis = FrequencyAnalysis::from(&entry);
            analysis.analyze();
            display_output_total += analysis.decode_display_output();
        }
    }

    println!("{}", display_output_total);

    Ok(())
}
