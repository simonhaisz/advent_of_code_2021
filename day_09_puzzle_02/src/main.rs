mod basin;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::basin::MapScanner;

fn main() -> std::io::Result<()> {

    let file = File::open("./day_09_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut scanner = MapScanner::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            let row = entry.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            scanner.scan_row(row);
        }
    }

    scanner.merge_basins();

    println!("{}", scanner.largest_basins_score(3));

    Ok(())
}
