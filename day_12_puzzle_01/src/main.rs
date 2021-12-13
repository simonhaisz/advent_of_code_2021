mod cave_network;

use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {

    let file = File::open("./day_12_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut connections = vec![];

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            connections.push(entry);
        }
    }

    let network = cave_network::create_cave_network(connections.iter().map(|s| s.as_str()).collect());
    let paths = cave_network::find_paths(network["start"].clone(), network["end"].clone());

    println!("{}", paths.len());

    Ok(())
}