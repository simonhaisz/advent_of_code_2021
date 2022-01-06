use std::collections::HashMap;
use crate::die;

pub type PlayerState = (u16, u16);
pub type PlayerMultiverse = HashMap<PlayerState, u64>;

pub fn get_all_player_states(starting_position: u16) -> Vec<PlayerMultiverse> {
	let dice_states = die::dice_states();

	let mut all_player_states: Vec<PlayerMultiverse> = vec![
		HashMap::from([((starting_position, 0), 1)])
	];

	loop {
		let previous_player_states = all_player_states.last().unwrap().clone();
		let mut next_player_states = HashMap::new();
		for ((position, score), previous_count) in previous_player_states.into_iter() {
			if score >= 21 {
				// game is over, do not continue
				continue;
			}
			for (value, dice_count) in dice_states.iter() {
				let next_position = move_player(position, *value);
				let next_score = score + next_position;
				let next_count = next_player_states.entry((next_position, next_score)).or_insert(0);
				*next_count += previous_count * *dice_count as u64;
			}
		}
		let exit_loop = next_player_states.iter().filter(|((_, s), _)| *s < 21).count() == 0;
		all_player_states.push(next_player_states);
		if exit_loop {
			break;
		}
	}

	all_player_states
}

fn move_player(start_position: u16, movement: u16) -> u16 {
	let new_position = start_position + movement;
	if new_position <= 10 {
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
	}
}