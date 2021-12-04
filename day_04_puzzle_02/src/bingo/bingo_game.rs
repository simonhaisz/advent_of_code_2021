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

    pub fn everyones_a_winner(&mut self) -> Option<Vec<Winner>> {
        if let Some(ball) = &mut self.ball {
            let mut winners = vec!();

            for number in ball {
                let mut new_winners = vec!();
                for (i, card) in self.cards.iter_mut().enumerate() {
                    if card.bingo() {
                        // we don't allow multiple winners
                        continue;
                    }
                    if let Some(score) = card.number_called(number) {
                        winners.push(Winner::new(number, i, score));
                        new_winners.push(i);
                    }
                }
            }

            if winners.len() < self.cards.len() {
                eprintln!("Not everyone wins! After running through all the numbers in the ball {} cards still don't have a bingo.", self.cards.len() - winners.len());
            }

            if winners.len() > 0 {
                return Some(winners);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub struct Winner {
    winning_number: u32,
    board_index: usize,
    score: u32,
}

impl Winner {
    pub fn new(winning_number: u32, board_index: usize, score: u32) -> Winner {
        Winner {
            winning_number,
            board_index,
            score,
        }
    }

    pub fn winning_number(&self) -> u32 {
        self.winning_number
    }

    pub fn board_index(&self) -> usize {
        self.board_index
    }

    pub fn score(&self) -> u32 {
        self.score
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

        let winners = game.everyones_a_winner();

        assert_eq!(
            Some(vec![
                Winner::new(24, 2, 4512),
                Winner::new(16, 0, 2192),
                Winner::new(13, 1, 1924),
            ]),
            winners
        );

    }
}