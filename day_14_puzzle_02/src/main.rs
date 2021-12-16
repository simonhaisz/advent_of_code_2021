mod polymer;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::polymer::{PolymerCounts, PairInsertionRule};

fn main() -> std::io::Result<()> {
    let file = File::open("./day_14_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut template = None;
    let mut rules = vec![];

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            if template.is_none() {
                template = Some(String::from(entry.trim()));
            } else {
                rules.push(String::from(entry.trim()));
            }
        }
    }

    let rules: Vec<PairInsertionRule> = rules.iter().map(|r| PairInsertionRule::from(r)).collect();

    let template = &template.unwrap();
    let mut polymer = PolymerCounts::from(template);

    for _ in 1..=10 {
        polymer = polymer.apply(&rules);
    }

    let score = polymer.score();

    println!("{}", score);

    Ok(())
}
