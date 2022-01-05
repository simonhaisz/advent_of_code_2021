mod die;
mod player;
mod game;

use crate::die::Die;
use crate::player::Player;

fn main() -> std::io::Result<()> {
    let mut die = Die::new();
    let mut player_1 = Player::new(8);
    let mut player_2 = Player::new(10);

    game::play_game(&mut die, &mut player_1, &mut player_2);

    let (winner, looser) = if player_1.winner() {
        (player_1, player_2)
    } else if player_2.winner() {
        (player_2, player_1)
    } else {
        panic!("Neither player 1 or player 2 has won the game")
    };

    println!("{} > {}", winner.score(), looser.score());
    println!("{}", looser.score() as u32 * die.roll_count());

    Ok(())
}
