

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down
}



#[derive(Copy, Clone)]
pub struct Point {
	pub x: usize,
	pub y: usize 
}

#[derive(PartialEq ,Copy, Clone)]
pub enum GridState {
	None,
	SnakeBody,
	SnakeHead,
	Apple
}