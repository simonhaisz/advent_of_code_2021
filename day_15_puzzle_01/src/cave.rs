use std::cmp;

pub struct CaveBuilder {
	width: Option<usize>,
	risk_levels: Vec<Vec<u32>>,
}

impl CaveBuilder {
	pub fn new() -> CaveBuilder {
		CaveBuilder {
			width: None,
			risk_levels: vec![],
		}
	}

	pub fn add_row(&mut self, row: Vec<u32>) {
		if let Some(width) = self.width {
			if width != row.len() {
				panic!("Row does not match the expected length of {} - found {}", width, row.len());
			}
		} else {
			self.width = Some(row.len());
		}

		self.risk_levels.push(row);
	}

	pub fn build(self) -> Cave {
		Cave::new(self.risk_levels)
	}
}

pub struct Cave {
	risk_levels: Vec<Position>,
	width: usize,
	height: usize,
}

impl Cave {
	pub fn new(risk_grid: Vec<Vec<u32>>) -> Cave {
		let mut risk_levels = vec![];
		for (y, row) in risk_grid.iter().enumerate() {
			for (x, risk) in row.iter().enumerate() {
				risk_levels.push(Position::new(x, y, *risk));
			}
		}
		Cave {
			risk_levels,
			width: risk_grid[0].len(),
			height: risk_grid.len(),
		}
	}

	pub fn start(&self) -> &Position {
		self.get_pos(0, 0)
	}

	pub fn end(&self) -> &Position {
		self.get_pos(self.width - 1, self.height - 1)
	}

	fn get_pos(&self, x: usize, y: usize) -> &Position {
		let index = x + y * self.width;
		&self.risk_levels[index]
	}

	pub fn neighbors(&self, pos: &Position) -> Vec<&Position> {
		let mut neighbors = vec![];

		if pos.x > 0 {
			neighbors.push(self.get_pos(pos.x - 1, pos.y));
		}
		if pos.x < self.width - 1 {
			neighbors.push(self.get_pos(pos.x + 1, pos.y));
		}
		if pos.y > 0 {
			neighbors.push(self.get_pos(pos.x, pos.y - 1));
		}
		if pos.y < self.height - 1 {
			neighbors.push(self.get_pos(pos.x, pos.y + 1));
		}

		neighbors
	}
}

#[derive(Eq, PartialEq, Hash)]
pub struct Position {
	x: usize,
	y: usize,
	risk: u32,
}

impl Position {
	pub fn new(x: usize, y: usize, risk: u32) -> Position {
		Position {
			x,
			y,
			risk,
		}
	}

	pub fn risk(&self) -> u32 {
		self.risk
	}

	pub fn distance(&self, other: &Self) -> usize {
		(cmp::max(self.x, other.x) - cmp::min(self.x, other.x)) + (cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
	}
}