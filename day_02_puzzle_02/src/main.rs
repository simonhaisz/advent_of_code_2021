mod submarine;

use std::fs::File;
use std::io::{BufReader, BufRead};
use submarine::Submarine;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_02_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut sub = Submarine::new();
    for line in lines {
        sub.execute_command(&line.expect("Expected a line"));
    }

    println!("Submarines ends up at horizontal position {} and depth {}", sub.horizontal_position(), sub.depth());
    println!("Multiplied together they are {}", sub.horizontal_position() * sub.depth());
    Ok(())
}
