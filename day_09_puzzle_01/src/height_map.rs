pub struct LowPoint {
	column: usize,
	row: usize,
	value: u32,
	risk_level: u32,
}

impl LowPoint {
	pub fn value(&self) -> u32 {
		self.value
	}

	pub fn risk_level(&self) -> u32 {
		self.risk_level
	}
}

pub struct HeightMap {
	size: Option<usize>,
	current_row_index: Option<usize>,
	map_complete: bool,
	prev_row: Option<Vec<u32>>,
	current_row: Option<Vec<u32>>,
	next_row: Option<Vec<u32>>,
	low_points: Vec<LowPoint>,
}

impl HeightMap {
	pub fn new() -> HeightMap {
		HeightMap {
			size: None,
			current_row_index: None,
			map_complete: false,
			prev_row: None,
			current_row: None,
			next_row: None,
			low_points: vec![],
		}
	}

	pub fn process_row(&mut self, row: Option<Vec<u32>>) {
		if self.map_complete {
			panic!("Map is complete, cannot process any more rows");
		}

		if let Some(row) = row.as_ref() {
			if let Some(size) = self.size {
				if size != row.len() {
					panic!("Map has a size of {}, cannot process row of size {}", size, row.len());
				}
				if self.current_row_index.is_none() {
					self.current_row_index = Some(0);
				}
			} else {
				self.size = Some(row.len());
			}
		}

		self.prev_row = self.current_row.take();
		self.current_row = self.next_row.take();
		self.next_row = row;

		if self.current_row.is_none() {
			// special case
			// cannot find low points with a single row - skip processing
		} else {
			let size = self.size.unwrap();
			let current_row = self.current_row.as_ref().unwrap();
			let current_row_index = self.current_row_index.unwrap();
			for column in 0..size {
				let value = current_row[column];
				if column > 0 {
					let left_value = current_row[column - 1];
					if value >= left_value {
						continue;
					}
				}
				if column < size -1 {
					let right_value = current_row[column + 1];
					if value >= right_value {
						continue;
					}
				}
				if let Some(prev_row) = self.prev_row.as_ref() {
					let top_value = prev_row[column];
					if value >= top_value {
						continue;
					}
				}
				if let Some(next_row) = self.next_row.as_ref() {
					let bottom_value = next_row[column];
					if value >= bottom_value {
						continue;
					}
				}
				self.low_points.push(LowPoint { column, row: current_row_index, value, risk_level: value + 1 });
			}

			if self.next_row.is_none() {
				self.map_complete = true;
			}
		}
	}

	pub fn risk_level_total(&self) -> u32 {
		self.low_points.iter().map(|lp| lp.risk_level).sum()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_demo() {
		let mut map = HeightMap::new();

		map.process_row(Some(vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]));
		map.process_row(Some(vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1]));
		map.process_row(Some(vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2]));
		map.process_row(Some(vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9]));
		map.process_row(Some(vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]));
		map.process_row(None);

		let lowest_point_values = map.low_points.iter().map(|lp| lp.value).collect::<Vec<u32>>();
		assert_eq!(vec![1, 0, 5, 5], lowest_point_values);
	}
}