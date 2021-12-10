#[macro_use]
extern crate lazy_static;

mod chunk;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::chunk::ChunkChecker;

fn main() -> std::io::Result<()> {

    let file = File::open("./day_10_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut checker = ChunkChecker::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            checker.parse_line(entry.trim());
        }
    }

    println!("{}", checker.middle_incomplete_score());

    Ok(())
}