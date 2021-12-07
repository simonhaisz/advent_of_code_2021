pub fn parse_numbers(input: &str) -> Vec<i32> {
    let mut numbers = input
        .split(",")
        .filter(|v| !v.trim().is_empty())
        .map(|v| i32::from_str_radix(v.trim(), 10).unwrap())
        .collect::<Vec<i32>>();
    
    numbers.sort();

    numbers
}

fn median(numbers: &Vec<i32>) -> i32 {
    let midpoint = (numbers.len() as f64 / 2.0).trunc() as usize - 1;
    if numbers.len() % 2 == 0 {
        // even
        let midpoint_a = midpoint;
        let midpoint_b = midpoint_a + 1;
        let a = numbers[midpoint_a];
        let b = numbers[midpoint_b];
        ((a + b) as f64 / 2.0).round() as i32
    } else {
        // odd
        numbers[midpoint + 1]
    }
}

fn fuel_cost(offset: i32) -> i32 {
    // each point of offset costs its value in fuel
    // eg. offset 3 > 1 + 2 + 3 = 6 fuel
    offset * (offset + 1) / 2
}

fn offset_total(numbers: &Vec<i32>, target: i32) -> i64 {
    let mut offset_total = 0;

    for n in numbers.iter() {
        let offset = (target - n).abs();
        offset_total += fuel_cost(offset) as i64;
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
    fn test_midpoint() {
        // odd
        let numbers = vec![2,4,4,6,12,37,87,99,185];
        assert_eq!(12, median(&numbers));

        // even
        let numbers = vec![2,4,4,6,12,37,87,99,185,1337];
        assert_eq!(25, median(&numbers));
    }

    #[test]
    fn test_offset_total() {
        let numbers = vec![0,1,1,2,2,2,4,7,14,16];

        assert_eq!(206, offset_total(&numbers, 2));
        assert_eq!(168, offset_total(&numbers, 5));
    }

    #[test]
    fn test_min_offset() {
        let numbers = vec![0,1,1,2,2,2,4,7,14,16];

        assert_eq!((5,168), min_offset_total_target_brute_force(&numbers));
    }
}