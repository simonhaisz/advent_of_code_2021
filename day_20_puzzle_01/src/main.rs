#[macro_use]
extern crate lazy_static;

mod pixel;

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::pixel::Image;

fn main() -> std::io::Result<()> {
    let file = File::open("./day_20_puzzle_01/input.txt")?;
    let lines = BufReader::new(file).lines();

    let mut enhancement: Option<String> = None;
    let mut pixels = vec![];
    
    for (index, line) in lines.enumerate() {
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        if index == 0 {
            enhancement = Some(line);
        } else {
            pixels.push(line);
        }
    }

    let enhancement = enhancement.unwrap();

    let image = Image::new(pixels);
    println!("{}", image.lit_pixel_count());

    let enhanced_image = image.enhance(&enhancement);
    println!("{}", enhanced_image.lit_pixel_count());

    let enhanced_enhanced_image = enhanced_image.enhance(&enhancement);
    println!("{}", enhanced_enhanced_image.lit_pixel_count());

    Ok(())
}
