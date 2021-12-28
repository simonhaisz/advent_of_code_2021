#[macro_use]
extern crate lazy_static;

mod cube;
mod range;
mod reactor;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::reactor::Reactor;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_22_puzzle_01/input.txt")?;

    let lines = BufReader::new(file).lines();

    let mut reactor = Reactor::new();

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            reactor = reactor.run_command(&entry.trim());
        }
    }

    println!("{}", reactor.cube_count(false));

    Ok(())
}
