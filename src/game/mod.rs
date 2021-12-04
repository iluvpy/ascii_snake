mod snake;
mod structs;
mod util;
mod apple;

use structs::*;
use std::io::Write;

pub const GRID_WIDTH: usize = 25;
pub const GRID_HEIGHT: usize = 25;

pub struct GameData {
	pub running: bool,
	pub snake: snake::Snake,
	pub apple: Point,
	pub grid: Vec<Vec<GridState>>,
	pub input: String
}

pub fn get_center_pos() -> Point {
	Point {
		x: (GRID_WIDTH/2) as usize,
		y: (GRID_HEIGHT/2) as usize
	}
}

pub fn init_game_data() -> GameData {
	let grid: Vec<Vec<GridState>> = vec![vec![GridState::None; GRID_WIDTH]; GRID_HEIGHT];
	let center = get_center_pos(); // get center of grid
	let mut snake_body: Vec<Point> = vec![];
	for x in center.x-5..center.x+1 {
		snake_body.push(Point{
			x: x,
			y: 0
		});
	}
	/* the snake body needs to be sorted from start (closest to head or the element that comes after the head) to
	   tail or last element so it needs to be inverted as the first element is currently the tail  
	*/
	snake_body = snake_body.into_iter().rev().collect(); 
	GameData {
		running: true,
		snake: snake::Snake {
			body: snake_body,
			head: Point {
				x: center.x,
				y: 0
			},
			direction: Direction::Right,
			alive: true
		},
		apple: get_center_pos(),
		grid: grid,
		input: String::from(" ")
	}
}


pub fn game_loop(game_data: &mut GameData) {
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout();
	while game_data.running {
		util::clear_console(); // clear console
		util::new_lines(25); // prints out a space so that the grid is not at the top of the console
		snake::update_snake(game_data); // update position
		apple::update_apple(game_data);
		apple::add_apple(game_data); // adds apple position to game grid
		snake::add_snake(game_data); // adds the snake body and head to game grid
		util::draw_grid(&game_data.grid); // 'draws' grid to console
		util::clear_grid(&mut game_data.grid); // sets everything in the grid to GridState::None 
		print!("enter movement w-a-s-d: ");
		stdout.flush();
		game_data.input.clear();
		stdin.read_line(&mut game_data.input).expect("Error occured while reading input");
	}
}


