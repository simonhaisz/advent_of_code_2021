mod die;
mod player;
mod game;

fn main() -> std::io::Result<()> {
    let player_1_states = player::get_all_player_states(4);
    let player_2_states = player::get_all_player_states(8);

    let (player_1_wins, player_2_wins) = game::determine_player_wins(&player_1_states, &player_2_states);

    println!("Player 1 ({} wins) Player 2 ({} wins)", player_1_wins, player_2_wins);
    Ok(())
}
