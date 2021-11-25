use super::structs::GridState;

pub fn draw_grid(grid: &Vec<Vec<GridState>>) {
	for vec_ in grid {
		for state in vec_ {
			let char_: char = match state {
				GridState::SnakeBody => '🟩',
				GridState::Apple => '🟥',
				_ => '⬜',
			};
			print!("{}", char_);

		}
		print!("\n");
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