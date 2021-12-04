use crate::bingo::bingo_ball::BingoBall;
use crate::bingo::bingo_card::BingoCard;

pub struct BingoGame {
    ball: Option<BingoBall>,
    cards: Vec<BingoCard>,
}

impl BingoGame {
    pub fn new() -> BingoGame {
        BingoGame {
            ball: None,
            cards: vec!(),
        }
    }

    pub fn setup_ball(&mut self, ball: BingoBall) {
        self.ball = Some(ball);
    }

    pub fn add_card(&mut self, card: BingoCard) {
        self.cards.push(card);
    }

    pub fn play_game(&mut self) -> Option<Vec<(u32, usize, u32)>> {
        if let Some(ball) = &mut self.ball {
            for number in ball {
                let mut winners = vec!();

                for (i, card) in self.cards.iter_mut().enumerate() {
                    if let Some(score) = card.number_called(number) {
                        winners.push((number, i, score));
                    }
                }

                if winners.len() > 0 {
                    return Some(winners);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut game = BingoGame::new();

        let ball = BingoBall::new("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1");

        game.setup_ball(ball);

        let mut card = BingoCard::new();
        card.load_row("22 13 17 11  0");
        card.load_row(" 8  2 23  4 24");
        card.load_row("21  9 14 16  7");
        card.load_row(" 6 10  3 18  5");
        card.load_row(" 1 12 20 15 19");
        assert_eq!(true, card.is_full());

        game.add_card(card);

        let mut card = BingoCard::new();
        card.load_row(" 3 15  0  2 22");
        card.load_row(" 9 18 13 17  5");
        card.load_row("19  8  7 25 23");
        card.load_row("20 11 10 24  4");
        card.load_row("14 21 16 12  6");
        assert_eq!(true, card.is_full());

        game.add_card(card);

        let mut card = BingoCard::new();
        card.load_row("14 21 17 24  4");
        card.load_row("10 16 15  9 19");
        card.load_row("18  8 23 26 20");
        card.load_row("22 11 13  6  5");
        card.load_row(" 2  0 12  3  7");
        assert_eq!(true, card.is_full());

        game.add_card(card);

        let winners = game.play_game();

        assert_eq!(Some(vec![(24, 2, 4512)]), winners);

    }
}