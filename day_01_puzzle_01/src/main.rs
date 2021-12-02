mod sonar_scan;

use std::fs::File;
use std::io::{BufReader, BufRead};
use sonar_scan::SonarScan;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_01_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut scan = SonarScan::new();
    for line in lines {
        let depth = line.expect("Error reading line").trim().parse().expect("Line should be an integer");
        scan.process_depth(depth);
    }

    println!("{}", scan.depth_increase_count());
    Ok(())
}
