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
	incomplete_opens: Vec<String>,
}

impl ChunkChecker {
	pub fn new() -> ChunkChecker {
		ChunkChecker {
			incomplete_opens: vec![],
		}
	}

	pub fn parse_line(&mut self, line: &str) {
		if let Some(opens) = find_incomplete_opens(&line) {
			self.incomplete_opens.push(opens);
		}
	}

	pub fn middle_incomplete_score(&self) -> u64 {
		let mut scores = vec![];

		for opens in self.incomplete_opens.iter() {
			let mut score = 0;
			for c in opens.chars() {
				score *= 5;
				score += match c {
					')' => 1,
					']' => 2,
					'}' => 3,
					'>' => 4,
					_ => panic!("Expected invalid character to be ')', ']', '}}', or '>' - found '{}'", c),
				}
			}

			scores.push(score);
		}

		scores.sort();

		// assumption that there is always an odd number of incomplete lines
		assert_eq!(1, scores.len() % 2);

		scores[scores.len() / 2]
	}
}

pub fn find_incomplete_opens(line: &str) -> Option<String> {
	let mut opens = vec![];
	'chars: for c in line.chars() {
		for (open, close) in OPEN_CLOSE_PAIRS.iter() {
			if c == *open {
				opens.push(c);
				continue 'chars;
			} else if c == *close {
				let last_index = opens.len() - 1;
				let last_open = opens[last_index];
				let expected_close = OPEN_CLOSE_PAIRS.get(&last_open).unwrap();
				if expected_close == close {
					opens.remove(last_index);
				} else {
					// illegal character - skip this line
					return None;
				}
			}
		}
	}

	if opens.len() > 0 {
		let mut missing_closes = vec![];
		for open in opens.iter().rev() {
			let close = OPEN_CLOSE_PAIRS.get(&open).unwrap();
			missing_closes.push(*close);
		}
		Some(String::from_iter(missing_closes))
	} else {
		None
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_find_incomplete_opens_none() {
		assert_eq!(None, find_incomplete_opens("[<>({}){}[([])<>]]"));
	}

	#[test]
	fn test_first_incomplete_opens_1() {
		assert_eq!("}}]])})]", find_incomplete_opens("[({(<(())[]>[[{[]{<()<>>").unwrap());
	}

	#[test]
	fn test_first_incomplete_opens_2() {
		assert_eq!(")}>]})", find_incomplete_opens("[(()[<>])]({[<{<<[]>>(").unwrap());
	}


	#[test]
	fn test_first_incomplete_opens_3() {
		assert_eq!("}}>}>))))", find_incomplete_opens("(((({<>}<{<{<>}{[]{[]{}").unwrap());
	}

	#[test]
	fn test_first_incomplete_opens_4() {
		assert_eq!("]]}}]}]}>", find_incomplete_opens("{<[[]]>}<{[{[{[]{()[[[]").unwrap());
	}

	#[test]
	fn test_first_incomplete_opens_5() {
		assert_eq!("])}>", find_incomplete_opens("<{([{{}}[<[[[<>{}]]]>[]]").unwrap());
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

		assert_eq!(288957, checker.middle_incomplete_score());
	}
}