mod paper;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use crate::paper::PaperBuilder;

fn main() -> std::io::Result<()> {

    let dot_regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    let fold_regex = Regex::new(r"fold along (?P<axis>[xy])=(?P<offset>\d+)").unwrap();

    let file = File::open("./day_13_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut builder = PaperBuilder::new();

    let mut fold_commands: Vec<String> = vec![];

    for line in lines {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                continue;
            }
            if dot_regex.is_match(entry.trim()) {
                if let Some(captures) = dot_regex.captures(entry.trim()) {
                    builder.add_dot(
                        usize::from_str_radix(&captures["x"], 10).unwrap(),
                        usize::from_str_radix(&captures["y"], 10).unwrap(),
                    );
                } else {
                    panic!("Matched as dot but could not capture as dot - '{}'", entry.trim())
                }
            } else if fold_regex.is_match(entry.trim()) {
                fold_commands.push(String::from(entry.trim()));
            } else {
                panic!("Unexpected input is not dot or fold command - found '{}'", entry.trim())
            }
        }
    }

    let paper = builder.build();

    let folded_paper =
    if let Some(captures) = fold_regex.captures(&fold_commands[0]) {
        let offset = usize::from_str_radix(&captures["offset"], 10).unwrap();
        match &captures["axis"] {
            "x" => paper.fold_vertical(offset),
            "y" => paper.fold_horizontal(offset),
            _ => panic!("Unexpected fold axis - '{}'", &captures["axis"])
        }
    } else {
        panic!("Matched as command but could not capture as command - '{}'", fold_commands[0])
    };

    println!("{}", folded_paper.len());

    Ok(())
}