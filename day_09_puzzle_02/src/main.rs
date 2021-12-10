mod basin;
mod height_map;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::height_map::HeightMap;

fn main() -> std::io::Result<()> {

    let file = File::open("./day_09_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut map = HeightMap::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            let row = entry.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
            map.process_row(Some(row));
        }
    }
    map.process_row(None);

    println!("{}", map.risk_level_total());

    Ok(())
}
