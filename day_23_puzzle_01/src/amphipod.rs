pub enum Amphipod {
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl Amphipod {
	pub fn from(input: &str) -> Amphipod {
		match input {
			"A" => Amphipod::Amber,
			"B" => Amphipod::Bronze,
			"C" => Amphipod::Copper,
			"D" => Amphipod::Desert,
			_ => panic!("Invalid amphipod - expected A, B, C, or D found '{}'", input)
		}
	}

	pub fn energy(&self) -> u32 {
		match self {
			Amphipod::Amber => 1,
			Amphipod::Bronze => 10,
			Amphipod::Copper => 100,
			Amphipod::Desert => 1000,
		}
	}
}