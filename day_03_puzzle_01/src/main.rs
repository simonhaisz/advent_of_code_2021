mod power_consumption;

use std::fs::File;
use std::io::{BufReader, BufRead};
use power_consumption::PowerConsumption;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_03_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut power = PowerConsumption::new();
    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().len() == 0 {
                // skip any rows with no content
                continue;
            }
            power.analyze_entry(&entry.trim());
        }
    }

    println!("The submarine's diagnostics give it a gamma rate of {} and an epsilon rate of {}", power.gamme_rate(), power.epsilon_rate());
    println!("Multiplied together that gives {}", power.gamme_rate() * power.epsilon_rate());

    Ok(())
}
