use std::cmp;
use crate::range::{self, CubeRange};

#[derive(Debug, PartialEq)]
pub struct Cuboid {
    ranges: Vec<CubeRange>,
}

const THREE_D_LENGTH: usize = 3;
const THREE_D_RANGE: std::ops::Range<usize> = 0..3;
const THREE_D_OTHER_D: [[usize; 2]; 3] = [
    [2, 1],
    [0, 2],
    [1, 0],
];

fn apply_area_limit(r: &CubeRange) -> Option<CubeRange> {
    if *r.start() > 50 || *r.end() < -50 {
        None
    } else {
        Some(
            cmp::max(*r.start(), -50)..=cmp::min(*r.end(), 50)
        )
    }
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
        if ranges.len() != THREE_D_LENGTH {
            panic!("Cuboid must have exactly 3 dimensions - found {}", ranges.len())
        }
        Cuboid {
            ranges
        }
    }

    pub fn area(&self) -> i64 {
        let limited_area_ranges = self.ranges
            .iter()
            .map(|r| apply_area_limit(r))
            .filter(|r| r.is_some())
            .map(|r| r.unwrap()).
            collect::<Vec<_>>();
        
        // if any dimension is fully outside the limit then the whole thing is
        if limited_area_ranges.len() == 3 {
            limited_area_ranges.iter().map(|r| (r.end() - r.start() + 1) as i64).product()
        } else {
            0
        }
        // without the limit (why did they give a limit?)
        // self.ranges.iter().map(|r| (r.end() - r.start() + 1) as i64).product()
    }

    pub fn intersection(&self, other: &Cuboid) ->Option<Cuboid> {
        let mut inters = vec![];
        for d in THREE_D_RANGE {
            let inter = range::intersection(&self.ranges[d], &other.ranges[d]);
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

    pub fn subtract(self, other: &Cuboid) -> CuboidSet {
        if let Some(inter) = self.intersection(other) {
            let mut splits = vec![];
            // if complete overlap then are left with nothing
            if inter != self {
                let x = 0;
                let y = 1;
                let z = 2;
                if self.ranges[x].start() < inter.ranges[x].start() {
                    let bottom = Cuboid::new(
                        *self.ranges[x].start()..=(inter.ranges[x].start() - 1),
                        self.ranges[y].clone(),
                        self.ranges[z].clone(),
                    );
                    splits.push(bottom);
                }
                {
                    let x_range = inter.ranges[x].clone();

                    let get_ranges = |o: &CubeRange, i: &CubeRange| -> Vec<Option<CubeRange>> {
                        let mut ranges: Vec<Option<CubeRange>> = vec![];
                        if o.start() < i.start() {
                            ranges.push(Some(*o.start()..=(i.start() - 1)))
                        } else {
                            ranges.push(None);
                        }
                        ranges.push(Some(i.clone()));
                        if o.end() > i.end() {
                            ranges.push(Some((i.end() + 1)..=*o.end()));
                        } else {
                            ranges.push(None);
                        }
                        ranges
                    };

                    let y_ranges = get_ranges(&self.ranges[y], &inter.ranges[y]);
                    let z_ranges = get_ranges(&self.ranges[z], &inter.ranges[z]);

                    for y_i in 0..3 {
                        for z_i in 0..3 {
                            // skip the center donut-hole
                            if y_i == 1 && z_i == 1 {
                                continue;
                            }
                            let y_range = y_ranges[y_i].as_ref();
                            let z_range = z_ranges[z_i].as_ref();
                            if y_range.is_some() && z_range.is_some() {
                                let cuboid = Cuboid::new(x_range.clone(), y_range.unwrap().clone(), z_range.unwrap().clone());
                                splits.push(cuboid);
                            }
                        }
                    }
                }
                if self.ranges[x].end() > inter.ranges[x].end() {
                    let top = Cuboid::new(
                        (inter.ranges[x].end() + 1)..=*self.ranges[x].end(),
                        self.ranges[y].clone(),
                        self.ranges[z].clone(),
                    );
                    splits.push(top);
                }
            }
            splits
        } else {
            vec![self]
        }
    }
}

pub type CuboidSet = Vec<Cuboid>;

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

    #[test]
    fn three_d_other_d() {
        assert_eq!(vec![2, 1], THREE_D_OTHER_D[0]);

        assert_eq!(vec![0, 2], THREE_D_OTHER_D[1]);

        assert_eq!(vec![1, 0], THREE_D_OTHER_D[2]);
    }

    #[test]
    fn cuboid_subtract_center() {
        let a = Cuboid::new(1..=3, 1..=3, 1..=3);
        let b = Cuboid::new(2..=2, 2..=2, 2..=2);

        let expected = vec![
            Cuboid::new(1..=1, 1..=3, 1..=3),

            Cuboid::new(2..=2, 1..=1, 1..=1),
            Cuboid::new(2..=2, 1..=1, 2..=2),
            Cuboid::new(2..=2, 1..=1, 3..=3),
            Cuboid::new(2..=2, 2..=2, 1..=1),
            Cuboid::new(2..=2, 2..=2, 3..=3),
            Cuboid::new(2..=2, 3..=3, 1..=1),
            Cuboid::new(2..=2, 3..=3, 2..=2),
            Cuboid::new(2..=2, 3..=3, 3..=3),

            Cuboid::new(3..=3, 1..=3, 1..=3),
        ];
        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    #[test]
    fn cuboid_subtract_corner() {
        let a = Cuboid::new(1..=10, 1..=10, 1..=10);
        let b = Cuboid::new(5..=15, 5..=15, 5..=15);

        let expected = vec![
            Cuboid::new(1..=4, 1..=10, 1..=10),

            Cuboid::new(5..=10, 1..=4, 1..=4),
            Cuboid::new(5..=10, 1..=4, 5..=10),
            Cuboid::new(5..=10, 5..=10, 1..=4),
        ];

        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    #[test]
    fn cuboid_subtract_half() {
        let a = Cuboid::new(-5..=5, -5..=5, -5..=5);
        let b = Cuboid::new(1..=10, -5..=5, -5..=5);

        let expected = vec![
            Cuboid::new(-5..=0, -5..=5, -5..=5),
        ];

        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    #[test]
    fn cuboid_subtract_side() {
        let a = Cuboid::new(100..=1000, 100..=1000, 100..=1000);
        let b = Cuboid::new(100..=199, 0..=2000, 50..=1337);

        let expected = vec![
            Cuboid::new(200..=1000, 100..=1000, 100..=1000),
        ];

        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    #[test]
    fn cuboid_subtract_disjoint() {
        let a = Cuboid::new(20..=30, 30..=40, 40..=50);
        let b = Cuboid::new(0..=10, 10..=20, 20..=30);

        let expected = vec![
            Cuboid::new(20..=30, 30..=40, 40..=50)
        ];

        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    #[test]
    fn cuboid_subtract_overlap() {
        let a = Cuboid::new(1..=5, 1..=5, 1..=5);
        let b = Cuboid::new(0..=10, 0..=10, 0..=10);

        let expected = vec![];

        let actual = a.subtract(&b);

        compare_cuboid_sets(&expected, &actual);
    }

    fn compare_cuboid_sets(left: &CuboidSet, right: &CuboidSet) {
        for c in left.iter() {
            assert_eq!(true, right.contains(&c), "Missing: {:?}", c.ranges);
        }

        for c in right.iter() {
            assert_eq!(true, left.contains(&c), "Extra: {:?}", c.ranges);
        }

        let sum = |s: &CuboidSet| -> i64 {
            s.iter().map(|c| c.area()).sum()
        };

        assert_eq!(sum(left), sum(right));
    }
}