type Position = (usize, usize);

trait AreAdjacent<T> {
	fn are_adjacent(&self, other: &T) -> bool;
}

impl AreAdjacent<Position> for Position {
	fn are_adjacent(&self, other: &Position) -> bool {
		let x_delta = (self.0 as isize - other.0 as isize).abs();
		let y_delta = (self.1 as isize - other.1 as isize).abs();
		// diagonals are not adjacent
		x_delta == 1 && y_delta == 0 || x_delta == 0 && y_delta == 1
	}
}

pub struct Basin {
	positions: Vec<Position>,
}

impl Basin {
	fn new(position: Position) -> Basin {
		Basin {
			positions: vec![position],
		}
	}

	fn merge(&mut self, other:&mut Basin) {
		self.positions.append(&mut other.positions);
	}

	fn clone(&self) -> Basin {
		let positions = self.positions.iter().map(|&p| p.clone()).collect::<Vec<Position>>();
		Basin {
			positions
		}
	}

	fn intersects(&self, other: &Basin) -> bool {
		for other_p in other.positions.iter() {
			if self.is_within(other_p) {
				return true;
			}
		}

		false
	}

	fn is_within(&self, position: &Position) -> bool {
		for p in self.positions.iter() {
			if p.are_adjacent(position) {
				return true;
			}
		}
		false
	}

	fn add(&mut self, position: Position) {
		self.positions.push(position);
	}

	pub fn len(&self) -> usize {
		self.positions.len()
	}
}

pub struct MapScanner {
	size: Option<usize>,
	current_row_index: Option<usize>,
	basins: Vec<Basin>,
}

impl MapScanner {
	pub fn new() -> MapScanner {
		MapScanner {
			size: None,
			current_row_index: None,
			basins: vec![],
		}
	}

	pub fn scan_row(&mut self, row: Vec<u32>) {
		if let Some(size) = self.size {
			if size != row.len() {
				panic!("Map has a size of {}, cannot process row of size {}", size, row.len());
			}
		} else {
			self.size = Some(row.len());
		}
		if let Some(row_index) = self.current_row_index.as_mut() {
			*row_index += 1;
		} else {
			self.current_row_index = Some(0);
		}

		'column: for (column, value) in row.iter().enumerate() {
			if *value == 9 {
				// skip all nines (they are the highest possible value)
				continue;
			}
			if *value > 9 {
				panic!("Expected highest value is 9 - found {}", value);
			}
			let position = (column, self.current_row_index.unwrap());
			for basin in self.basins.iter_mut() {
				if basin.is_within(&position) {
					basin.add(position);
					continue 'column;
				}
			}

			self.basins.push(Basin::new(position));
		}
	}

	pub fn merge_basins(&mut self) {
		let mut merged_basins = vec![];
		'outer: for outer in 0..self.basins.len() - 1 {
			let outer_basin = &self.basins[outer];
			'inner: for other_i in outer + 1..self.basins.len() {
				let other_basin = &self.basins[other_i];
				if outer_basin.intersects(&other_basin) {
					let mut merged_basin = outer_basin.clone();
					let mut copy = other_basin.clone();
					merged_basin.merge(&mut copy);
					merged_basins.push(merged_basin);
					continue 'outer;
				}
			}
		}

		self.basins = merged_basins;
	}

	pub fn largest_basins_score(&mut self, count: usize) -> usize {
		self.basins.sort_by(|a, b| a.positions.len().partial_cmp(&b.positions.len()).unwrap());
		self.basins.reverse();

		let mut score = 1;
		for i in 0..count {
			let basin = &self.basins[i];
			score *= basin.positions.len();
		}
		score
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_demo_find_basins() {
		let mut scanner = MapScanner::new();

		scanner.scan_row(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
		scanner.scan_row(vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1]);
		scanner.scan_row(vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2]);
		scanner.scan_row(vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9]);
		scanner.scan_row(vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]);

		scanner.merge_basins();

		assert_eq!(4, scanner.basins.len());
	}

	#[test]
	fn test_demo_largst_score() {
		let mut scanner = MapScanner::new();

		scanner.scan_row(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
		scanner.scan_row(vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1]);
		scanner.scan_row(vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2]);
		scanner.scan_row(vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9]);
		scanner.scan_row(vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]);

		scanner.merge_basins();

		assert_eq!(1134, scanner.largest_basins_score(3));
	}
}