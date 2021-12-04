use super::structs::*;
use super::structs::GridState;
use super::GameData;


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


pub fn update_snake(game_data: &mut GameData) {
	let mut snake = &mut game_data.snake;
	let mut new = snake.head;
	let char_: char = game_data.input.chars().nth(0).unwrap(); // get first char of input
	snake.direction = match char_ {
		'w' => Direction::Up,
		's' => Direction::Down,
		'd' => Direction::Right,
		'a' => Direction::Left,
		 _ => snake.direction,
	};


	let direction = snake.direction;
	if direction == Direction::Up {
		new.y = snake.head.y - 1;
	} else if direction == Direction::Down {
		new.y = snake.head.y + 1;
	} else if direction == Direction::Left {
		new.x = snake.head.x - 1;
	} else if direction == Direction::Right {
		new.x = snake.head.x + 1;
	}

	// move current element to the old position of the next element
	let mut old = snake.head;
	snake.head = new;
	for i in 0..snake.body.len() {
		let position_cpy = snake.body[i];
		print!("settings old: {} {} ", snake.body[i].x, snake.body[i].y); // XXX remove later
		snake.body[i] = old;
		println!("to -> {} {}", old.x, old.y);
		old = position_cpy;
	}

}