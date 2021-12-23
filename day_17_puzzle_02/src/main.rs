#[macro_use]
extern crate lazy_static;

mod geometry;
mod ballistics;
mod triangle_number;

use std::fs;
use crate::geometry::Rectangle;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./day_17_puzzle_01/input.txt").unwrap();

    let target = Rectangle::from(&input.trim());

    let all_hit_launches = ballistics::find_all_hit_launches(&target);
    println!("Found {} unique hit launches", all_hit_launches.len());

    Ok(())
}