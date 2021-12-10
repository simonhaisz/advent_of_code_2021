use std::collections::HashMap;
use array_tool::vec::Intersect;

pub struct FrequencyAnalysis<'input> {
	signal_patterns: Vec<&'input str>,
	digital_output: Vec<&'input str>,
	character_positions: HashMap<char, Vec<u32>>,
	position_characters: HashMap<u32, Vec<char>>,
}

lazy_static! {
	static ref ALL_CHARACTERS: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

	static ref ALL_POSITIONS: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6];
}

impl<'input> FrequencyAnalysis<'input> {
	pub fn new(signal_patterns: Vec<&'input str>, digital_output: Vec<&'input str>) -> FrequencyAnalysis<'input> {
		let mut character_positions = HashMap::new();
		for c in ALL_CHARACTERS.iter() {
			character_positions.insert(*c, ALL_POSITIONS.clone());
		}
		let mut position_characters = HashMap::new();
		for p in ALL_POSITIONS.iter() {
			position_characters.insert(*p, ALL_CHARACTERS.clone());
		}

		FrequencyAnalysis {
			signal_patterns,
			digital_output,
			character_positions,
			position_characters,
		}
	}

	fn reduce_character_positions(&mut self, character: char, positions: &Vec<u32>) {
		let current_positions = self.character_positions.get(&character).unwrap();
		let mut intersection_positions = vec![];
		for p in current_positions.iter() {
			if positions.contains(p) {
				intersection_positions.push(*p);
			}
		}
		self.character_positions.insert(character, intersection_positions);
	}

	fn reduce_position_characters(&mut self, position: u32, characters: &Vec<char>) {
		let current_characters = self.position_characters.get(&position).unwrap();
		let mut intersection_characters = vec![];
		for c in current_characters.iter() {
			if characters.contains(c) {
				intersection_characters.push(*c);
			}
		}
		self.position_characters.insert(position, intersection_characters);
	}

	pub fn analyze(&mut self) {
		let digits: Vec<&Digit> = vec![&ONE, &SEVEN];
		
		self.run_pass(&digits);

		let digits: Vec<&Digit> = vec![&TWO, &THREE, &FIVE];

		self.run_pass(&digits);

		let digits: Vec<&Digit> = vec![&ZERO, &SIX, &NINE];

		self.run_pass(&digits);
	}

	fn run_pass(&mut self, digits: &Vec<&Digit>) {
		let union_positions = compute_position_counts(&digits);
		
		println!("<position>: count");
		for (p, count) in union_positions.iter() {
			println!("{}: {}", p, count);
		}
		let mut matching_dignals: Vec<&str> = vec![];
		'signals: for signal in self.signal_patterns.iter() {
			for d in digits.iter() {
				if signal.len() == d.len() {
					matching_dignals.push(signal);
					continue 'signals;
				}
			}
		}
		let union_characters = compute_character_counts(matching_dignals);

		println!();

		println!("<character>: count");
		for (c, count) in union_characters.iter() {
			println!("{}: {}", c, count);
		}

		println!();

		let mut count_positions_characters = HashMap::new();
		for count in 1..=digits.len() {
			let count = count as u32;
			
			println!("<count>: <character>");
			let mut matching_characters = vec![];
			for (c, c_count) in union_characters.iter() {
				if count == *c_count {
					println!("{}: {}", count, c);
					matching_characters.push(*c);
				}
			}

			println!("<count>: <position>");
			let mut matching_positions = vec![];
			for (p, p_count) in union_positions.iter() {
				if count == *p_count {
					println!("{}: {}", count, p);
					matching_positions.push(*p);
				}
			}

			if matching_characters.len() == 0 && matching_characters.len() == 0 {
				continue;
			}

			count_positions_characters.insert(count, (matching_positions, matching_characters));
		}

		for (_, (positions, characters)) in count_positions_characters.iter() {

			for c in characters.iter() {
				self.reduce_character_positions(*c, positions);
			}

			for p in positions.iter() {
				self.reduce_position_characters(*p, characters);
			}
		}

		let mut solved_characters = vec![];
		for (character, positions) in self.character_positions.iter() {
			if positions.len() == 1 {
				solved_characters.push(*character);
			}
		}

		let mut solved_positions = vec![];
		for (position, characters) in self.position_characters.iter() {
			if characters.len() == 1 {
				solved_positions.push(*position);
			}
		}

		for (_, positions) in self.character_positions.iter_mut() {
			if positions.len() > 1 {
				positions.retain(|&p| !solved_positions.contains(&p));
			}
		}

		for (_, characters) in self.position_characters.iter_mut() {
			if characters.len() > 1 {
				characters.retain(|&c| !solved_characters.contains(&c));
			}
		}
	}

	pub fn output_frequency(&self) {
		println!("<character>: [positions]");
		for (c, positions) in self.character_positions.iter() {
			println!("{}: {}", c, positions.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", "));
		}

		println!();

		println!("<position>: [characters]");
		for (p, characters) in self.position_characters.iter() {
			println!("{}: {}", p, characters.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", "));
		}

		println!();
	}

	pub fn decoded_digits(&self) -> Vec<u32> {
		let mut decoded_digits = vec![];
		for encoded_digit in self.digital_output.iter() {
			let mut decoded_positions = vec![];
			for c in encoded_digit.chars() {
				let p = self.character_positions.get(&c).unwrap()[0];
				decoded_positions.push(p);
			}
			decoded_positions.sort();

			if let Some(digit) = find_digit(decoded_positions) {
				decoded_digits.push(digit.value);
			} else {
				panic!("Could not find matching digit encoded value '{}'", encoded_digit);
			}
		}

		decoded_digits
	}
}

pub struct Digit {
	value: u32,
	positions: Vec<u32>,
}

impl Digit {
	fn len(&self) -> usize {
		self.positions.len()
	}
}

fn compute_position_counts(digits: &Vec<&Digit>) -> HashMap<u32, u32> {
	let mut position_counts = HashMap::new();

	for d in digits.iter() {
		for p in d.positions.iter() {
			let c = position_counts.entry(*p).or_insert(0);
			*c += 1;
		}
	}

	position_counts
}

fn filter_to_unique_positions(position_counts: &HashMap<u32, u32>) -> Vec<(u32, u32)> {
	let mut unique_positions = vec![];

	'outer: for (position, count) in position_counts.iter() {
		'inner: for (other_position, other_count) in position_counts.iter() {
			if position == other_position {
				continue 'inner;
			}
			if count == other_count {
				continue 'outer;
			}
		}
		unique_positions.push((*position, *count));
	}

	unique_positions
}

fn compute_character_counts(signals: Vec<&str>) -> HashMap<char, u32> {
	let mut character_counts = HashMap::new();

	for signal in signals.iter() {
		for c in signal.chars() {
			let count = character_counts.entry(c).or_insert(0);
			*count += 1;
		}
	}

	character_counts
}

fn filter_to_uqniue_characters(character_counts: &HashMap<char, u32>) -> Vec<(char, u32)> {
	let mut unique_characters = vec![];

	'outer: for (character, count) in character_counts.iter() {
		'inner: for (other_character, other_count) in character_counts.iter() {
			if character == other_character {
				continue 'inner;
			}
			if count == other_count {
				continue 'outer;
			}
		}
		unique_characters.push((*character, *count));
	}

	unique_characters
}

lazy_static! {
	static ref ZERO: Digit = Digit { value: 0, positions: vec![0, 1, 2, 4, 5, 6] };
	static ref ONE: Digit = Digit { value: 1, positions: vec![2, 5] };
	static ref TWO: Digit = Digit { value: 2, positions: vec![0, 2, 3, 4, 6] };
	static ref THREE: Digit = Digit { value: 3, positions: vec![0, 2, 3, 5, 6] };
	static ref FOUR: Digit = Digit { value: 4, positions: vec![1, 2, 3, 5] };
	static ref FIVE: Digit = Digit { value: 5, positions: vec![0, 1, 3, 5, 6] };
	static ref SIX: Digit = Digit { value: 6, positions: vec![0, 1, 3, 4, 5, 6] };
	static ref SEVEN: Digit = Digit { value: 7, positions: vec![0, 2, 5] };
	static ref EIGHT: Digit = Digit { value: 8, positions: vec![0, 1, 2, 3, 4, 5, 6] };
	static ref NINE: Digit = Digit { value: 9, positions: vec![0, 1, 2, 3, 5, 6] };

	static ref ALL_DIGITS: HashMap<u32, Vec<&'static Digit>> =
	{
		let mut map: HashMap<u32, Vec<&Digit>> = HashMap::new();
	
		map.insert(2, vec![&ONE]);
		map.insert(3, vec![&SEVEN]);
		map.insert(4, vec![&FOUR]);
		map.insert(5, vec![&TWO, &THREE, &FIVE]);
		map.insert(6, vec![&ZERO, &SIX, &NINE]);
		map.insert(7, vec![&EIGHT]);

		map
	};
}

fn find_digit(positions: Vec<u32>) -> Option<&'static Digit> {
	for (_, digits) in ALL_DIGITS.iter() {
		'digits: for d in digits.iter() {
			if d.len() == positions.len() {
				for i in 0..positions.len() {
					if d.positions[i] != positions[i] {
						continue 'digits;
					}
				}
				return Some(d);
			}
		}
	}

	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_decode() {
		let signal_patterns = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb".split(" ").collect::<Vec<&str>>();
		let digital_output = "fdgacbe cefdb cefbgd gcbe".split(" ").collect::<Vec<&str>>();

		let mut analysis = FrequencyAnalysis::new(signal_patterns, digital_output);

		analysis.analyze();

		let decoded_digits = analysis.decoded_digits();

		assert_eq!(vec![8, 3, 9, 4], decoded_digits);
	}
}