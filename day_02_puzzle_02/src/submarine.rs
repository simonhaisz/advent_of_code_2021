pub struct Submarine {
	horizontal_position: i32,
	depth: i32,
	aim: i32,
}

impl Submarine {

	pub fn new() -> Submarine {
		Submarine {
			horizontal_position: 0,
			depth: 0,
			aim: 0,
		}
	}

	pub fn horizontal_position(&self) -> i32 {
		self.horizontal_position
	}

	pub fn depth(&self) -> i32 {
		self.depth
	}

	pub fn aim(&self) -> i32 {
		self.aim
	}

	fn forward(&mut self, value: i32) {
		self.horizontal_position += value;
		self.depth += self.aim * value;
	}

	fn down(&mut self, value: i32) {
		self.aim += value;
	}

	fn up(&mut self, value: i32) {
		self.aim -= value;
	}

	pub fn execute_command(&mut self, command: &str) {
		let command = Command::parse(&command);
		match command {
			Command::Forward(value) => self.forward(value),
			Command::Down(value) => self.down(value),
			Command::Up(value) => self.up(value),
		}
	}
}

#[derive(Debug, PartialEq)]
enum Command {
	Forward(i32),
	Down(i32),
	Up(i32),
}

impl Command {
	fn parse(command: &str) -> Command {
		let mut split = command.split_whitespace();
		let name = split.next().unwrap();
		let value = split.next().unwrap().trim().parse().expect("Command value should be an integer");
		match name {
			"forward" => Command::Forward(value),
			"down" => Command::Down(value),
			"up" => Command::Up(value),
			_ => panic!("Unknown command '{}' provided - expected 'forward', 'down', or 'up'", name),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_defaults() {
		let sub = Submarine::new();

		assert_eq!(0, sub.depth());
		assert_eq!(0, sub.horizontal_position());
		assert_eq!(0, sub.aim());
	}

	#[test]
	fn test_forward() {
		let mut sub = Submarine::new();
		sub.forward(10);
		sub.forward(5);
		sub.forward(2);

		assert_eq!(17, sub.horizontal_position());
		assert_eq!(0, sub.depth());
		assert_eq!(0, sub.aim());
	}

	#[test]
	fn test_down() {
		let mut sub = Submarine::new();
		sub.down(5);
		sub.down(2);

		assert_eq!(0, sub.depth());
		assert_eq!(0, sub.horizontal_position());
		assert_eq!(7, sub.aim());
	}

	#[test]
	fn test_up() {
		let mut sub = Submarine::new();
		sub.up(20);
		sub.up(1);

		assert_eq!(0, sub.depth());
		assert_eq!(0, sub.horizontal_position());
		assert_eq!(-21, sub.aim());
	}

	#[test]
	fn test_parse_forward() {
		let command = Command::parse("forward 1");
		assert_eq!(Command::Forward(1), command);
	}

	#[test]
	fn test_parse_down() {
		let command = Command::parse("down 5");
		assert_eq!(Command::Down(5), command);
	}

	#[test]
	fn test_parse_up() {
		let command = Command::parse("up 13");
		assert_eq!(Command::Up(13), command);
	}

	#[test]
	fn test_triginometry() {
		let mut sub = Submarine::new();
		sub.down(10);
		sub.forward(2);
		sub.up(5);
		sub.forward(5);

		assert_eq!(45, sub.depth());
		assert_eq!(7, sub.horizontal_position());
		assert_eq!(5, sub.aim());
	}
}