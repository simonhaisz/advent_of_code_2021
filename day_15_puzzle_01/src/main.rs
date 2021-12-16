mod cave;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::cave::CaveBuilder;
fn main() -> std::io::Result<()> {
    let file = File::open("./day_15_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut builder = CaveBuilder::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            builder.add_row(
                entry.trim()
                    .chars()
                    .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                    .collect::<Vec<u32>>()
            );
        }
    }

    let cave = builder.build();

    let safest_path = cave.find_safest_path_recursive().unwrap();

    println!("{}", safest_path.risk());

    Ok(())
}
