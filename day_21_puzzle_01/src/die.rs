use std::iter::Iterator;

pub struct Die {
	next_value: u8,
	roll_count: u32,
}

impl Die {
	pub fn new() -> Die {
		Die {
			next_value: 1,
			roll_count: 0,
		}
	}

	pub fn roll_count(&self) -> u32 {
		self.roll_count
	}
}

impl Iterator for Die {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		let value = self.next_value;
		let next_value = if value < 100 {
			value + 1
		} else {
			1
		};
		self.next_value = next_value;
		self.roll_count += 1;
		Some(value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default() {
		let mut die = Die::new();
		assert_eq!(0, die.roll_count);
		assert_eq!(Some(1), die.next());
		assert_eq!(1, die.roll_count);
	}

	#[test]
	fn roll() {
		let mut die = Die::new();
		for roll in 1..=100 {
			assert_eq!(Some(roll), die.next());
			assert_eq!(roll, die.roll_count as u8);
		}
		assert_eq!(Some(1), die.next());
		assert_eq!(101, die.roll_count as u8);
	}
}