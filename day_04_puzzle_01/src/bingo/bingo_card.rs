pub struct BingoCard {
    size: usize,
    numbers: Vec<u32>,
}

impl BingoCard {
    pub fn new() -> BingoCard {
        BingoCard {
            size: 0,
            numbers: vec!()
        }
    }

    pub fn load_row(&mut self, row_numbers: &str) {
        let row = &mut parse_row_numbers(row_numbers);
        if self.size == 0 {
            self.size = row.len();
        } else if self.size != row.len() {
            panic!("Bingo card of size {} cannot take a row of size {}", self.size, row.len());
        } else if self.is_full() {
            panic!("Bingo card is full ({} rows of length {}) and cannot take any more rows", self.size, self.size);
        }
        self.numbers.append(row);
    }

    pub fn is_full(&self) -> bool {
        if self.size == 0 {
            false
        } else {
            self.numbers.len() >= self.size * self.size
        }
    }
}

fn parse_row_numbers(row_numbers: &str) -> Vec<u32> {
    row_numbers.split(" ")
        .filter(|&s| !s.is_empty())
        .map(|n| u32::from_str_radix(n, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let row = parse_row_numbers(" 2 13 14  8 74");
        assert_eq!(vec![2, 13, 14, 8, 74], row);
    }

    #[test]
    fn test_build_card() {
        let mut card = BingoCard::new();
        assert_eq!(false, card.is_full());
        card.load_row("22 13 17 11  0");
        assert_eq!(false, card.is_full());
        card.load_row(" 8  2 23  4 24");
        assert_eq!(false, card.is_full());
        card.load_row("21  9 14 16  7");
        assert_eq!(false, card.is_full());
        card.load_row(" 6 10  3 18  5");
        assert_eq!(false, card.is_full());
        card.load_row(" 1 12 20 15 19");
        assert_eq!(true, card.is_full());
    }

    #[test]
    #[should_panic(expected = "Bingo card of size 5 cannot take a row of size 3")]
    fn test_too_small() {
        let mut card = BingoCard::new();
        card.load_row(" 1 16 31 46 61");
        card.load_row(" 2 17 32");
    }

    #[test]
    #[should_panic(expected = "Bingo card of size 3 cannot take a row of size 5")]
    fn test_too_big() {
        let mut card = BingoCard::new();
        card.load_row(" 2 17 32");
        card.load_row(" 1 16 31 46 61");
    }

    #[test]
    #[should_panic(expected = "Bingo card is full (3 rows of length 3) and cannot take any more rows")]
    fn test_card_already_full() {
        let mut card = BingoCard::new();
        card.load_row(" 1 16 31");
        card.load_row(" 7 22 37");
        card.load_row("15 30 45");
        card.load_row("16 21 46");
    }
}