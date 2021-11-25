use super::structs::GridState;

pub fn draw_grid(grid: &Vec<Vec<GridState>>) {
	for vec_ in grid {
		for state in vec_ {
			let char_: char = match state {
				GridState::SnakeBody => '🟩',
				GridState::Apple => '🟥',
				GridState::SnakeHead => '🟢',
				_ => '⬜',
			};
			print!("{}", char_);

		}
		print!("\n");
	}
}

pub fn clear_grid(grid: &mut Vec<Vec<GridState>>) {
	for i in 0..grid.len() {
		for j in 0..grid[i].len() {
			grid[i][j] = GridState::None;
		}
	}
}

pub fn clear_console() {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn new_lines(n: i32) {
	for _ in 0..n {
		print!("\n");
	}
}