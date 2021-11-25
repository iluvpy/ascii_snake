mod snake;
mod structs;
mod util;
mod apple;

use structs::*;
use std::time::Duration;
use std::thread::sleep;

pub const GRID_WIDTH: usize = 25;
pub const GRID_HEIGHT: usize = 25;

pub struct GameData {
	pub running: bool,
	pub snake: snake::Snake,
	pub apple: Point,
	pub grid: Vec<Vec<GridState>>
}

pub fn get_center_pos() -> Point {
	Point {
		x: (GRID_WIDTH/2) as usize,
		y: (GRID_HEIGHT/2) as usize
	}
}

pub fn init_game_data() -> GameData {
	let grid: Vec<Vec<GridState>> = vec![vec![GridState::None; GRID_WIDTH]; GRID_HEIGHT];
	let center = get_center_pos();
	// for i in 0..GRID_HEIGHT {
	// 	let mut tmp: Vec<GridState> = vec![];
	// 	for j in 0..GRID_WIDTH {
	// 		tmp.push(GridState::None);
	// 	}
	// 	grid.push(tmp);
	// }
	let mut snake_body: Vec<Point> = vec![];
	for x in center.x-5..center.x+2 {
		snake_body.push(Point{
			x: x,
			y: 0
		});
	}
	GameData {
		running: true,
		snake: snake::Snake {
			body: snake_body,
			head: Point {
				x: center.x,
				y: center.y
			},
			direction: Direction::Right,
			alive: true
		},
		apple: get_center_pos(),
		grid: grid
	}
}

pub fn game_loop(game_data: &mut GameData) {
	while game_data.running {
		sleep(Duration::from_secs(1));
		util::clear_console();
		util::new_lines(25);
		snake::update_snake(&mut game_data.snake);
		apple::add_apple(game_data);
		snake::add_snake(game_data);
		util::draw_grid(&game_data.grid);
		util::clear_grid(&mut game_data.grid);
	}
}


