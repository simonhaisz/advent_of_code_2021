#[macro_use]
extern crate lazy_static;

mod line;
mod grid;
mod geometry;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::grid::Grid;
use crate::line::Line;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_05_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut grid = Grid::new();
    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().len() == 0 {
                // skip any rows with no content
                continue;
            }
            let l = Line::from(entry.trim());
            grid.add_line(l);
        }
    }

    let overlaps = grid.overlaps();

    println!("With all of the lines there are {} points where they overlap", overlaps.len());

    Ok(())
}

