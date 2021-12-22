#[macro_use]
extern crate lazy_static;

mod geometry;
mod ballistics;
mod triangle_number;

use std::cmp;
use std::fs;
use crate::geometry::Rectangle;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./day_17_puzzle_01/input.txt").unwrap();

    let target = Rectangle::from(&input.trim());

    let fanciest_hit = ballistics::find_fanciest_hit_arc(&target);

    if let Some(hit) = fanciest_hit {
        let mut max_y_pos = 0;
        for p in hit.iter() {
            max_y_pos = cmp::max(max_y_pos, p.p_y());
        }

        println!("{}", max_y_pos);

    } else {
        panic!("No hit was found for target area")
    }

    Ok(())
}