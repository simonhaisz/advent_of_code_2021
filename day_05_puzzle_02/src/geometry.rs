use std::cmp;

pub fn overlap(a_min: i32, a_max: i32, b_min: i32, b_max: i32) -> bool {
    b_min <= a_max && b_max >= a_min
}

pub fn overlap_range(a_min: i32, a_max: i32, b_min: i32, b_max: i32) -> Option<(i32, i32)> {
    if !overlap(a_min, a_max, b_min, b_max) {
        return None
    }

    let start = cmp::max(a_min, b_min);
    let end = cmp::min(a_max, b_max);

    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_before_b() {
        assert_eq!(false, overlap(1, 10, 11, 20));
    }

    #[test]
    fn test_b_before_a() {
        assert_eq!(false, overlap(11, 20, 1, 10));
    }

    #[test]
    fn test_a_overlap_b() {
        assert_eq!(true, overlap(1, 10, 5, 15));
    }

    #[test]
    fn test_b_overlap_a() {
        assert_eq!(true, overlap(5, 15, 1, 10));
    }

    #[test]
    fn test_a_within_b() {
        assert_eq!(true, overlap(3, 7, 1, 10));
    }

    #[test]
    fn test_b_within_a() {
        assert_eq!(true, overlap(1, 10, 3, 7));
    }

    #[test]
    fn test_a_overlap_range_b() {
        assert_eq!((5, 10), overlap_range(1, 10, 5, 15).unwrap());
    }

    #[test]
    fn test_b_overlap_range_a() {
        assert_eq!((5, 10), overlap_range(5, 15, 1, 10).unwrap());
    }

    #[test]
    fn test_a_within_range_b() {
        assert_eq!((3, 7), overlap_range(3, 7, 1, 10).unwrap());
    }

    #[test]
    fn test_b_within_range_a() {
        assert_eq!((3, 7), overlap_range(1, 10, 3, 7).unwrap());
    }
}