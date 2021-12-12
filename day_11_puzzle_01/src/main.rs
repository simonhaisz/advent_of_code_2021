mod octopus_grid;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::octopus_grid::OctopusGrid;

fn main() -> std::io::Result<()> {

    let file = File::open("./day_11_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut grid = OctopusGrid::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            grid.add_row(entry.trim().chars()
            .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
            .collect::<Vec<u32>>());
        }
    }

    println!("{}", grid.compute_flashes(100));

    Ok(())
}