mod bingo;

use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::bingo::{BingoGame, BingoBall, BingoCard};

fn main() -> std::io::Result<()> {
    let file = File::open("./day_04_puzzle_01/input.txt")?;
    let reader = BufReader::new(file);

    let mut game = BingoGame::new();

    let mut current_card = None;

    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.trim().len() == 0 {
                // skip empty lines
                continue;
            }
            if i == 0 {
                let ball = BingoBall::new(line.trim());
                game.setup_ball(ball);
            } else {
                if current_card.is_none() {
                    current_card = Some(BingoCard::new());
                }
                if let Some(c) = &mut current_card {
                    c.load_row(line.trim());
                    if c.is_full() {
                        game.add_card(current_card.take().unwrap());
                    }
                }
            }
        }
    }

    if let Some(winners) = game.everyones_a_winner() {
        for winner in winners.iter() {
            println!("Calling number {} gave board {} a Bingo with a score of {}", winner.winning_number(), winner.board_index(), winner.score());
        }
    } else {
        println!("We ran out of numbers before anyone got a Bingo!");
    }

    Ok(())
}
