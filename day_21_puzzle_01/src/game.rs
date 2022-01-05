use crate::die::Die;
use crate::player::Player;

pub fn play_game(die: &mut Die, player_1: &mut Player, player_2: &mut Player) {
	loop {
		if player_1.take_turn(die) {
			break;
		}
		if player_2.take_turn(die) {
			break;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		let mut die = Die::new();

		let mut player_1 = Player::new(4);
		let mut player_2 = Player::new(8);

		play_game(&mut die, &mut player_1, &mut player_2);

		assert_eq!(1000, player_1.score());
		assert_eq!(745, player_2.score());
		assert_eq!(993, die.roll_count());
	}
}