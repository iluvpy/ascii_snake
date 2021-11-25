mod game;

fn main() {
    let mut game_data = game::init_game_data();
	game::game_loop(&mut game_data);
}
