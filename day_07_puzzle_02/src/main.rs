mod crab_alignment;

use std::fs;

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("./day_07_puzzle_01/input.txt").unwrap();

    let numbers = crab_alignment::parse_numbers(&data);
    let min_offset = crab_alignment::min_offset_total_target_brute_force(&numbers);

    println!("The minimum amount of fuel to align all of the crabs (at position {}) is {}", min_offset.0, min_offset.1);

    Ok(())
}
