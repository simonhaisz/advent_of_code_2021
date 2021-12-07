mod school_of_fish;

use std::fs;
use crate::school_of_fish::SchoolOfFish;

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./day_06_puzzle_01/input.txt").unwrap();

    let mut school = SchoolOfFish::from(data.trim());

    // for _ in 0..80 {
    for _ in 0..256 {
        school.next_day();
    }

    println!("After 80 days there are {} lanternfish", school.len());

    Ok(())
}
