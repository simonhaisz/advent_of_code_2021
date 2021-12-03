extern crate stats_alloc;

mod life_support;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::life_support::LifeSupport;

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() -> std::io::Result<()> {
    let reg = Region::new(&GLOBAL);

    let file = File::open("./day_03_puzzle_01/input.txt")?;
    let reader = BufReader::new(file);

    let mut set_size = false;

    let mut life_support = LifeSupport::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().len() > 0 {
                if !set_size {
                    life_support.set_size(line.trim().len());
                }
                let data = u32::from_str_radix(&line.trim(), 2).unwrap();
                life_support.load_diagnostic(data);
            }
        }
    }

    println!("Stats after processing data: {:#?}", reg.change());

    let oxygen = life_support.oxygen_rating();
    let scrubber = life_support.scrubber_rating();

    println!("Stats after computing ratings: {:#?}", reg.change());

    println!("The submarine's diagnostics give it an oxygen rating of {} and a scrubber rating of {}", oxygen, scrubber);
    println!("Multiplied together that gives {}", oxygen * scrubber);

    println!("Stats after completing: {:#?}", reg.change());

    Ok(())
}
