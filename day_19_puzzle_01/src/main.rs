mod location;
mod scanner;

use std::collections::HashMap;
use crate::location::{Location, Angle, Rotation};

fn main() {
    let all_angles = [Angle::Zero, Angle::Ninety, Angle::OneEighty, Angle::TwoSeventy];

    let location = [10, 20, 30];

    let mut unique_rotations: HashMap<Location, Vec<Rotation>> = HashMap::new();

    let mut count = 0;
    for x in all_angles.iter() {
        for y in all_angles.iter() {
            for z in all_angles.iter() {
                count += 1;
                let rotation = [*x, *y, *z];
                let rotated = location::rotate_by(&location, &rotation);

                println!("{}: {:?}", count, rotated);

                unique_rotations.entry(rotated).or_insert(Vec::new()).push(rotation);
            }
        }
    }

    for (l, r) in unique_rotations.into_iter() {
        println!("{:?} from {:?}", l, r);
    }
}
