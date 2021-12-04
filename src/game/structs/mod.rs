

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
	Left = 97,
	Right = 100,
	Up = 119,
	Down = 115
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