use std::ops::RangeInclusive;
use std::cmp;

pub type CubeRange = RangeInclusive<i32>;

pub fn intersection(a: &CubeRange, b: &CubeRange) -> Option<CubeRange> {
    let (first, second) = if a.start() <= b.start() {
        (a, b)
    } else {
        (b, a)
    };

    let start = cmp::max(first.start(), second.start());
    let end = cmp::min(first.end(), second.end());

    if start <= end {
        Some(*start..=*end)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap() {
        let a = 1..=10;
        let b = 11..=20;
        assert_eq!(None, intersection(&a, &b));

        let a = 1000..=1001;
        let b = -5..=-1;
        assert_eq!(None, intersection(&a, &b));
    }

    #[test]
    fn overlap() {
        let a = 100..=105;
        let b = 103..=1000;
        assert_eq!(Some(103..=105), intersection(&a, &b));

        let a = 5000..=5500;
        let b = 1..=5000;
        assert_eq!(Some(5000..=5000), intersection(&a, &b));
    }

    #[test]
    fn within() {
        let a = -99..=100;
        let b = -9..=10;
        assert_eq!(Some(-9..=10), intersection(&a, &b));

        let a = 13..=13;
        let b = 0..=1000;
        assert_eq!(Some(13..=13), intersection(&a, &b));
    }
    
    #[test]
    fn its_a_circle() {
        let a = 50..=75;
        let b = 50..=75;
        assert_eq!(Some(50..=75), intersection(&a, &b));
    }
}