use crate::range::{self, CubeRange};

#[derive(Debug, PartialEq)]
pub struct Cuboid {
    ranges: Vec<CubeRange>,
}

impl Cuboid {
    pub fn new(x: CubeRange, y: CubeRange, z: CubeRange) -> Cuboid {
        Cuboid {
            ranges: vec![
                x,
                y,
                z,
            ]
        }
    }

    pub fn from(ranges: Vec<CubeRange>) -> Cuboid {
        if ranges.len() != 3 {
            panic!("Cuboid must have exactly 3 dimensions - found {}", ranges.len())
        }
        Cuboid {
            ranges
        }
    }

    pub fn area(&self) -> i32 {
        self.ranges.iter().map(|r| r.end() - r.start() + 1).product()
    }

    pub fn intersection(&self, other: &Cuboid) ->Option<Cuboid> {
        let mut inters = vec![];
        for i in 0..3 {
            let inter = range::intersection(&self.ranges[i], &other.ranges[i]);
            if inter.is_some() {
                inters.push(inter.unwrap());
            }
        }
        if inters.len() == 3 {
            Some(Cuboid::from(inters))
        } else {
            None
        }
    }

    pub fn subtract(self, other: &Cuboid) -> Option<CuboidSet> {
        if let Some(inter) = self.intersection(other) {
            todo!()
        } else {
            None
        }
    }
}

type CuboidSet = Vec<Cuboid>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cuboid_area() {
        let three_cubed = Cuboid::new(10..=12, 10..=12, 10..=12);
        assert_eq!(27, three_cubed.area());

        let flat_square = Cuboid::new(1..=10, 11..=20, -9..=-9);
        assert_eq!(100, flat_square.area());

        let irregular_cube = Cuboid::new(13..=37, -5..=5, 0..=12);
        assert_eq!(3575, irregular_cube.area());
    }

    #[test]
    fn cuboid_disjoint() {
        let a = Cuboid::new(10..=12, 10..=12, 10..=12);
        let b = Cuboid::new(20..=22, 20..=22, 20..=22);
        assert_eq!(None, a.intersection(&b));

        let b = Cuboid::new(10..=12, 1..=100, 1..=5);
        assert_eq!(None, b.intersection(&a));
    }

    #[test]
    fn cuboid_overlap() {
        let a = Cuboid::new(1..=10, 1..=10, 1..=10);
        let b = Cuboid::new(5..=15, 5..=15, 5..=15);
        assert_eq!(Some(Cuboid::new(5..=10, 5..=10, 5..=10)), a.intersection(&b));
    }

    #[test]
    fn cuboid_within() {
        let a = Cuboid::new(100..=200, 200..=300, 300..=400);
        let b = Cuboid::new(120..=170, 220..=270, 320..=370);

        assert_eq!(Some(Cuboid::new(120..=170, 220..=270, 320..=370)), b.intersection(&a));
    }
}