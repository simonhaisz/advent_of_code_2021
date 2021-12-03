mod life_support;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::life_support::LifeSupport;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_03_puzzle_01/input.txt")?;
    let reader = BufReader::new(file);

    let mut entries: Vec<String> = vec!();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().len() > 0 {
                entries.push(String::from(line.trim()));
            }
        }
    }
    let mut life_support = LifeSupport::new();
    for entry in entries.iter() {
        if entry.trim().len() == 0 {
            // skip any rows with no content
            continue;
        }
        life_support.load_diagnostic(&entry.trim());
    }

    let oxygen = life_support.oxygen_rating();
    let scrubber = life_support.scrubber_rating();

    println!("The submarine's diagnostics give it an oxygen rating of {} and a scrubber rating of {}", oxygen, scrubber);
    println!("Multiplied together that gives {}", oxygen * scrubber);

    Ok(())
}
