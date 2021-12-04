use super::structs::GridState;
use super::GameData;


pub fn add_apple(game_data: &mut GameData) {
	let x = game_data.apple.x;
	let y = game_data.apple.y;
	game_data.grid[y][x] = GridState::Apple;
}	

pub fn update_apple(game_data: &mut GameData) {
	
}