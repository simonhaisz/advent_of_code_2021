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
		let height = self.risk_levels.len();
		Cave::new(self.risk_levels, self.width.unwrap(), height)
	}
}

pub struct Cave {
	risk_levels: Vec<Vec<u32>>,
	width: usize,
	height: usize,
}

impl Cave {
	pub fn new(risk_levels: Vec<Vec<u32>>, width: usize, height: usize) -> Cave {
		Cave {
			risk_levels,
			width,
			height,
		}
	}

	pub fn find_safest_path(&self) -> Path {
		let mut all_paths: Vec<Path> = vec![Path::new()];

		let mut paths_to_add: Vec<Path> = vec![];

		let mut safest_path: Option<Path> = None;

		let mut iteration_count = 0;

		while all_paths.len() > 0 {
			for path in all_paths.iter() {
				let current_position = path.positions.last().unwrap();
				'moves: for next_move in current_position.valid_moves(&self).into_iter() {
					let next_position = next_move.next_position(&current_position);
					let new_risk = self.risk_levels[next_position.1][next_position.0];
					let mut new_path = path.clone();
					if new_path.add(new_risk, next_position) {
						if let Some(safest) = safest_path.as_ref() {
							if new_path.risk >= safest.risk {
								continue 'moves;
							}
						}
						if next_position.at_end(self) {
							safest_path = Some(new_path);
						} else {
							paths_to_add.push(new_path);
						}
					}
				}
			}
			all_paths = paths_to_add;
			paths_to_add = vec![];
			iteration_count += 1;
			println!("Completed iteration {}", iteration_count);
		}

		safest_path.unwrap()
	}
}

type Position = (usize, usize);

trait PositionInCave {
	fn valid_moves(&self, cave: &Cave) -> Vec<Move>;
	fn at_end(&self, cave: &Cave) -> bool;
}

impl PositionInCave for Position {

	fn valid_moves(&self, cave: &Cave) -> Vec<Move> {
		let mut moves = vec![];
		if self.0 > 0 {
			moves.push(Move::Left);
		}
		if self.0 < cave.width - 1 {
			moves.push(Move::Right);
		}
		if self.1 > 0 {
			moves.push(Move::Down);
		}
		if self.1 < cave.height - 1 {
			moves.push(Move::Up);
		}
		moves
	}

	fn at_end(&self, cave: &Cave) -> bool {
		self.0 == cave.width -1 && self.1 == cave.height - 1
	}
}

enum Move {
	Up,
	Down,
	Left,
	Right
}

impl Move {
	pub fn next_position(&self, position: &Position) -> Position {
		match self {
			Move::Up => (position.0, position.1 + 1),
			Move::Down => (position.0, position.1 - 1),
			Move::Left => (position.0 - 1, position.1),
			Move::Right => (position.0 + 1, position.1),
		}
	}
}

#[derive(Clone)]
pub struct Path {
	risk: u32,
	positions: Vec<Position>,
}

impl Path {
	pub fn new() -> Path {
		Path {
			risk: 0,
			positions: vec![(0, 0)],
		}
	}

	pub fn add(&mut self, new_risk: u32, p: Position) -> bool {
		if self.positions.contains(&p) {
			false
		} else {
			self.risk += new_risk;
			self.positions.push(p);
			true
		}
	}

	pub fn risk(&self) -> u32 {
		self.risk
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_15_demo() {
		let rows = "
		1163751742
		1381373672
		2136511328
		3694931569
		7463417111
		1319128137
		1359912421
		3125421639
		1293138521
		2311944581"
			.trim()
            .split("\n")
            .map(|r| r.trim().chars()
                .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>();

		let mut builder = CaveBuilder::new();
		for row in rows.into_iter() {
			builder.add_row(row);
		}

		let cave = builder.build();

		let path = cave.find_safest_path();

		assert_eq!(40, path.risk);
	}
}