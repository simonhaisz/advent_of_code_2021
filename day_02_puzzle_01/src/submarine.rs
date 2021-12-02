pub struct Submarine {
	position: i32,
	depth: i32,
}

impl Submarine {

	pub fn new() -> Submarine {
		Submarine {
			position: 0,
			depth: 0,
		}
	}

	pub fn position(&self) -> i32 {
		self.position
	}

	pub fn depth(&self) -> i32 {
		self.depth
	}

	pub fn forward(&mut self, delta: i32) {
		self.position += delta;
	}

	pub fn down(&mut self, delta: i32) {
		self.depth += delta;
	}

	pub fn up(&mut self, delta: i32) {
		self.depth -= delta;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_defaults() {
		let sub = Submarine::new();

		assert_eq!(0, sub.depth());
		assert_eq!(0, sub.position());
	}

	#[test]
	fn test_forward() {
		let mut sub = Submarine::new();
		sub.forward(10);
		sub.forward(5);
		sub.forward(2);

		assert_eq!(17, sub.position());
		assert_eq!(0, sub.depth());
	}

	#[test]
	fn test_down() {
		let mut sub = Submarine::new();
		sub.down(5);
		sub.down(2);

		assert_eq!(7, sub.depth());
		assert_eq!(0, sub.position());
	}

	#[test]
	fn test_up() {
		let mut sub = Submarine::new();
		sub.up(20);
		sub.up(1);

		assert_eq!(-21, sub.depth());
		assert_eq!(0, sub.position());
	}
}