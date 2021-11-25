use super::structs::*;
use super::structs::GridState;
use super::GameData;

pub struct Snake {
	pub body: Vec<Point>,
	pub head: Point,
	pub direction: Direction
}


pub fn add_snake(game_data: &mut GameData) {
	for point in game_data.snake.body.iter() {
		let x = point.x;
		let y = point.y;
		game_data.grid[y][x] = GridState::SnakeBody;
	}
}


pub fn update_snake(snake: &mut Snake) {
	println!("updated snake!");
}