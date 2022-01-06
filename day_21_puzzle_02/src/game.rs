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
		let mut p1_count = 0;
		let mut p1_win_count = 0;
		let mut p1_non_win_count = 0;
		for ((_, score), count) in player_1_state.unwrap().iter() {
			p1_count += count;
			if *score >= 21 {
				p1_win_count += count;
			} else {
				p1_non_win_count += count;
			}
		}
		let mut p2_count = 0;
		let mut p2_win_count = 0;
		let mut p2_non_win_count = 0;
		for ((_, score), count) in player_2_state.unwrap().iter() {
			p2_count += count;
			if *score >= 21 {
				p2_win_count += count;
			} else  {
				p2_non_win_count += count;
			}
		}

		player_1_wins += p1_win_count;
	}
	(player_1_wins,	player_2_wins)
}