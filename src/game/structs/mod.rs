
pub enum Direction {
	Left,
	Right,
	Up,
	Down
}

pub struct Point {
	pub x: usize,
	pub y: usize
}

#[derive(Clone)]
pub enum GridState {
	None,
	SnakeBody,
	SnakeHead,
	Apple
}