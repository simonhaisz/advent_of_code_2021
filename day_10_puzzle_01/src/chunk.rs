use std::collections::HashMap;

lazy_static! {
	static ref OPEN_CLOSE_PAIRS: HashMap<char, char> = {
		let mut map = HashMap::new();
		map.insert('(', ')');
		map.insert('[', ']');
		map.insert('{', '}');
		map.insert('<', '>');

		map
	};
}

pub struct ChunkChecker {
	invalid_characters: Vec<char>,
}

impl ChunkChecker {
	pub fn new() -> ChunkChecker {
		ChunkChecker {
			invalid_characters: vec![],
		}
	}

	pub fn parse_line(&mut self, line: &str) {
		if let Some(c) = first_illegal_character(&line) {
			self.invalid_characters.push(c);
		}
	}

	pub fn syntax_error_score(&self) -> u32 {
		let mut score = 0;

		for c in self.invalid_characters.iter() {
			score +=  match *c {
				')' => 3,
				']' => 57,
				'}' => 1197,
				'>' => 25137,
				_ => panic!("Expected invalid character to be ')', ']', '}}', or '>' - found '{}'", c),
			}
		}

		score
	}
}

pub fn first_illegal_character(line: &str) -> Option<char> {
	let mut opens = vec![];
	for c in line.chars() {
		for (open, close) in OPEN_CLOSE_PAIRS.iter() {
			if c == *open {
				opens.push(c);
				continue;
			} else if c == *close {
				let last_index = opens.len() - 1;
				let last_open = opens[last_index];
				let expected_close = OPEN_CLOSE_PAIRS.get(&last_open).unwrap();
				if expected_close == close {
					opens.remove(last_index);
				} else {
					return Some(*close);
				}
			}
		}
	}

	None
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_first_illegal_characters_none() {
		assert_eq!(None, first_illegal_character("[({(<(())[]>[[{[]{<()<>>"));
	}

	#[test]
	fn test_first_illegal_characters_squiggle() {
		assert_eq!(Some('}'), first_illegal_character("{([(<{}[<>[]}>{[]{[(<()>"));
	}

	#[test]
	fn test_first_illegal_characters_round() {
		assert_eq!(Some(')'), first_illegal_character("[[<[([]))<([[{}[[()]]]"));
	}

	#[test]
	fn test_first_illegal_characters_square() {
		assert_eq!(Some(']'), first_illegal_character("[{[{({}]{}}([{[{{{}}([]"));
	}

	#[test]
	fn test_first_illegal_characters_angled() {
		assert_eq!(Some('>'), first_illegal_character("<{([([[(<>()){}]>(<<{{"));
	}

	#[test]
	fn test_demo() {
		let mut checker = ChunkChecker::new();
		checker.parse_line("[({(<(())[]>[[{[]{<()<>>");
		checker.parse_line("[(()[<>])]({[<{<<[]>>(");
		checker.parse_line("{([(<{}[<>[]}>{[]{[(<()>");
		checker.parse_line("(((({<>}<{<{<>}{[]{[]{}");
		checker.parse_line("[[<[([]))<([[{}[[()]]]");
		checker.parse_line("[{[{({}]{}}([{[{{{}}([]");
		checker.parse_line("{<[[]]>}<{[{[{[]{()[[[]");
		checker.parse_line("[<(<(<(<{}))><([]([]()");
		checker.parse_line("<{([([[(<>()){}]>(<<{{");
		checker.parse_line("<{([{{}}[<[[[<>{}]]]>[]]");

		assert_eq!(26397, checker.syntax_error_score());
	}
}