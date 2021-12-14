pub fn parse_numbers(input: &str) -> Vec<i32> {
    let mut numbers = input
        .split(",")
        .filter(|v| !v.trim().is_empty())
        .map(|v| i32::from_str_radix(v.trim(), 10).unwrap())
        .collect::<Vec<i32>>();
    
    numbers.sort();

    numbers
}

fn offset_total(numbers: &Vec<i32>, target: i32) -> i64 {
    let mut offset_total = 0;

    for n in numbers.iter() {
        let offset = (target - n).abs();
        offset_total += offset as i64;
    }

    offset_total
}

pub fn min_offset_total_target_brute_force(numbers: &Vec<i32>) -> (i32, i64) {
    let mut min_offset = i32::MAX;
    let mut min_offset_total = i64::MAX;

    for n in 0..numbers[numbers.len()-1] {
        let offset_total = offset_total(&numbers, n);
        if offset_total < min_offset_total {
            min_offset_total = offset_total;
            min_offset = n;
        } else {
            break;
        }
    }
    (min_offset, min_offset_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let numbers = parse_numbers("12,6,87,4,2,99,37,4,185");
        assert_eq!(vec![2,4,4,6,12,37,87,99,185], numbers);
    }

    #[test]
    fn test_offset_total() {
        let numbers = vec![0,1,1,2,2,2,4,7,14,16];

        assert_eq!(49, offset_total(&numbers, 0));
        assert_eq!(41, offset_total(&numbers, 1));
        assert_eq!(37, offset_total(&numbers, 2));
        assert_eq!(71, offset_total(&numbers, 10));
    }

    #[test]
    fn test_min_offset() {
        let numbers = vec![0,1,1,2,2,2,4,7,14,16];

        assert_eq!((2,37), min_offset_total_target_brute_force(&numbers));
    }
}