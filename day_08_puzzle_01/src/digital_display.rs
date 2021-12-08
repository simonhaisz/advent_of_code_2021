pub fn extract_output_digits(input: &str) -> &str {
	let mut split = input.split("|");
	// signals, ignore for now
	split.next();
	if let Some(output) = split.next() {
		if let Some(v) = split.next() {
			panic!("Unexpected third entry after spliting '{}' on | - '{}'", input, v);
		}
		output.trim()
	} else {
		panic!("Failed to parse '{}'", input);
	}
}

pub fn count_known_digits(encoded_digits: &str) -> u32 {
	let mut count = 0;
	for d in encoded_digits.split(" ") {
		match d.trim().len() {
			2 | 3 | 4 | 7 => count += 1,
			_ => {}
		}
	}
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_count_known_digits_empty() {
		assert_eq!(0, count_known_digits(""));
	}

	#[test]
	fn test_count_known_digits_length_2() {
		assert_eq!(1, count_known_digits("aa"));
	}

	#[test]
	fn test_count_known_digits_length_3() {
		assert_eq!(1, count_known_digits("aaa"));
	}

	#[test]
	fn test_count_known_digits_length_4() {
		assert_eq!(1, count_known_digits("aaa4"));
	}

	#[test]
	fn test_count_known_digits_length_7() {
		assert_eq!(1, count_known_digits("aaaaaaa"));
	}

	#[test]
	fn test_count_known_digits_multiple() {
		assert_eq!(4, count_known_digits(" a aa aaa aaaa aaaaa aaaaaa aaaaaaa aaaaaaaa"));
	}

	#[test]
	fn test_extract_digits() {
		assert_eq!("fdgacbe cefdb cefbgd gcbe", extract_output_digits("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"));

		assert_eq!("fcgedb cgb dgebacf gc", extract_output_digits("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"));
		
		assert_eq!("cg cg fdcagb cbg", extract_output_digits("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"));

		assert_eq!("efabcd cedba gadfec cb", extract_output_digits("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"));

		assert_eq!("gecf egdcabf bgf bfgea", extract_output_digits("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"));

		assert_eq!("gebdcfa ecba ca fadegcb", extract_output_digits("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"));

		assert_eq!("cefg dcbef fcge gbcadfe", extract_output_digits("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"));

		assert_eq!("ed bcgafe cdgba cbgef", extract_output_digits("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"));

		assert_eq!("gbdfcae bgc cg cgb", extract_output_digits("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"));

		assert_eq!("fgae cfgab fg bagce", extract_output_digits("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"));
	}

	#[test]
	fn test_count_digits() {
		assert_eq!(2, count_known_digits("fdgacbe cefdb cefbgd gcbe"));

		assert_eq!(3, count_known_digits("fcgedb cgb dgebacf gc"));
		
		assert_eq!(3, count_known_digits("cg cg fdcagb cbg"));

		assert_eq!(1, count_known_digits("efabcd cedba gadfec cb"));

		assert_eq!(3, count_known_digits("gecf egdcabf bgf bfgea"));

		assert_eq!(4, count_known_digits("gebdcfa ecba ca fadegcb"));

		assert_eq!(3, count_known_digits("cefg dcbef fcge gbcadfe"));

		assert_eq!(1, count_known_digits("ed bcgafe cdgba cbgef"));

		assert_eq!(4, count_known_digits("gbdfcae bgc cg cgb"));

		assert_eq!(2, count_known_digits("fgae cfgab fg bagce"));
	}
}