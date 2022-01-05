use crate::die::Die;

pub struct Player {
	position: u8,
	score: u16,
}

impl Player {
	pub fn new(position: u8) -> Player {
		Player {
			position,
			score: 0,
		}
	}

	pub fn score(&self) -> u16 {
		self.score
	}

	pub fn take_turn(&mut self, die: &mut Die) -> bool {
		for _ in 0..3 {
			let roll = die.next().unwrap();
			let movement = roll % 10;
			let new_position = self.position + movement;
			let adjusted_position = if new_position <= 10 {
				new_position
			} else {
				let ten_multiples = {
					let p = new_position / 10;
					if new_position % 10 == 0 {
						p - 1
					} else {
						p
					}
				};
				new_position - ten_multiples * 10
			};
			self.position = adjusted_position;
		}
		self.score += self.position as u16;

		self.winner()
	}

	pub fn winner(&self) -> bool {
		self.score >= 1000
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_start() {
		let mut die = Die::new();

		let mut player_1 = Player::new(4);
		let mut player_2 = Player::new(8);

		player_1.take_turn(&mut die);
		assert_eq!(10, player_1.position);
		assert_eq!(10, player_1.score);

		player_2.take_turn(&mut die);
		assert_eq!(3, player_2.position);
		assert_eq!(3, player_2.score);

		player_1.take_turn(&mut die);
		assert_eq!(4, player_1.position);
		assert_eq!(14, player_1.score);

		player_2.take_turn(&mut die);
		assert_eq!(6, player_2.position);
		assert_eq!(9, player_2.score);

		player_1.take_turn(&mut die);
		assert_eq!(6, player_1.position);
		assert_eq!(20, player_1.score);

		player_2.take_turn(&mut die);
		assert_eq!(7, player_2.position);
		assert_eq!(16, player_2.score);

		player_1.take_turn(&mut die);
		assert_eq!(6, player_1.position);
		assert_eq!(26, player_1.score);

		player_2.take_turn(&mut die);
		assert_eq!(6, player_2.position);
		assert_eq!(22, player_2.score);
	}

	#[test]
	fn example_finish() {
		let mut die = Die::new();

		let mut player_1 = Player::new(4);
		let mut player_2 = Player::new(8);

		loop {
			if player_1.take_turn(&mut die) {
				break;
			}
			if player_2.take_turn(&mut die) {
				break;
			}
		}

		assert_eq!(1000, player_1.score);
		assert_eq!(745, player_2.score);
		assert_eq!(993, die.roll_count());
	}
}