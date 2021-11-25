use super::structs::*;
use super::structs::GridState;
use super::GameData;

use super::GRID_HEIGHT;
use super::GRID_WIDTH;

pub struct Snake {
	pub body: Vec<Point>,
	pub head: Point,
	pub direction: Direction,
	pub alive: bool
}


pub fn add_snake(game_data: &mut GameData) {
	for i in 0..game_data.snake.body.len() {
		let x = game_data.snake.body[i].x;
		let y = game_data.snake.body[i].y;
		game_data.grid[y][x] = GridState::SnakeBody;
	}

	let head_ = game_data.snake.head;
	game_data.grid[head_.y][head_.x] = GridState::SnakeHead;
}


pub fn update_snake(snake: &mut Snake) {
	let mut new = Point {
		x: 10,
		y: 0
	};

	let direction = snake.direction;
	if direction == Direction::Up {
		new.y = snake.head.y - 1;
	} else if direction == Direction::Down {
		new.y = snake.head.y + 1;
	} else if direction == Direction::Left {
		new.x = snake.head.x - 1;
	} else if direction == Direction::Right {
		println!("direction is right");
		new.x = snake.head.x + 1;
	}
	
	else {println!("didnt move")};
	
	println!("updating snake!");
	println!("new head pos: {} {}", new.x, new.y);
	snake.head = new;
	for i in 0..snake.body.len() {
		let position_cpy = snake.body[i];
		snake.body[i] = new;
		new = position_cpy;
	}

}