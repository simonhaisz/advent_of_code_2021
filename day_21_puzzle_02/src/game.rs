use crate::player::PlayerMultiverse;

pub fn determine_player_wins(player_1_states: &Vec<PlayerMultiverse>, player_2_states: &Vec<PlayerMultiverse>) -> (u64, u64) {
	let mut player_1_wins = 0;
	let mut player_2_wins = 0;
	for turn in 0..player_1_states.len() {
		let player_1_state = player_1_states.get(turn);
		let player_2_state = player_2_states.get(turn);
		if player_1_state.is_none() || player_2_state.is_none() {
			break;
		}
		let mut turn_non_wins = 0;
		for ((_, score), count) in player_1_state.unwrap().iter() {
			if *score >= 21 {
				player_1_wins += count;
			} else {
				turn_non_wins += count;
			}
		}
		for ((_, score), count) in player_2_state.unwrap().iter() {
			if *score >= 21 {
				player_2_wins += std::cmp::min(*count, turn_non_wins);
			}
		}
	}
	(player_1_wins,	player_2_wins)
}