pub struct BingoBall {
    numbers: std::vec::IntoIter<u32>,
}

impl BingoBall {
    pub fn new(seed_numbers: &str) -> BingoBall {
        let numbers = parse_seed_numbers(seed_numbers).into_iter();
        BingoBall {
            numbers,
        }
    }
}

impl Iterator for BingoBall {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.numbers.next()
    }
}

fn parse_seed_numbers(seed_numbers: &str) -> Vec<u32> {
    seed_numbers.split(",")
    .map(|n| u32::from_str_radix(n, 10).unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_value() {
        let numbers = parse_seed_numbers("13");
        assert_eq!(vec![13], numbers);
    }

    #[test]
    fn test_parse_multiple_values() {
        let numbers = parse_seed_numbers("1,17,44,98,27");
        assert_eq!(vec![1,17,44,98,27], numbers);
    }

    #[test]
    fn test_bingo_ball() {
        let mut ball = BingoBall::new("1,17,44,98,27");
        assert_eq!(Some(1), ball.next());
        assert_eq!(Some(17), ball.next());
        assert_eq!(Some(44), ball.next());
        assert_eq!(Some(98), ball.next());
        assert_eq!(Some(27), ball.next());
        assert_eq!(None, ball.next());

    }
}