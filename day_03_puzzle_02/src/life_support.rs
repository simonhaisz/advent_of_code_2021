pub struct LifeSupport<'input> {
	diagnostic_data: Vec<&'input str>,
}

impl<'input> LifeSupport<'input> {

	pub fn new() -> LifeSupport<'input> {
		LifeSupport {
			diagnostic_data: vec!()
		}
	}

	pub fn load_diagnostic(&mut self, data: &'input str) {
		if self.diagnostic_data.len() > 0 {
			if data.len() != self.diagnostic_data[0].len() {
				panic!("All diagnistic data must have the same length - {} does not match the size of first entry {}", data, self.diagnostic_data[0]);
			}
		}
		self.diagnostic_data.push(data);
	}

	pub fn oxygen_rating(&self) -> i32 {
		let mut current_diagnostics = self.diagnostic_data.clone();

		let mut index = 0;

		while current_diagnostics.len() > 0 {
			let mut current_summary = DiagnosticSummary::new();
			for diagnostic in current_diagnostics.iter() {
				current_summary.analyze_diagnostic(diagnostic);
			}

			let current_oxygen = current_summary.oxygen_rating(index);

			current_diagnostics = current_diagnostics
				.into_iter()
				.filter(|d| diagnostic_data_matches(d, current_oxygen, index))
				.collect();
			
			if current_diagnostics.len() == 0 {
				panic!("After filtering on index {} there are no diagnostics", index);
			} else if current_diagnostics.len() == 1 {
				break;
			}
			index += 1;

			if index >= current_diagnostics[0].len() {
				// can only scan to the last character
				break;
			}
		}

		convert_to_decimal(current_diagnostics[0])
	}

	pub fn scrubber_rating(&self) -> i32 {
		let mut current_diagnostics = self.diagnostic_data.clone();

		let mut index = 0;

		while current_diagnostics.len() > 0 {
			let mut current_summary = DiagnosticSummary::new();
			for diagnostic in current_diagnostics.iter() {
				current_summary.analyze_diagnostic(diagnostic);
			}

			let current_scrubber = current_summary.scrubber_rating(index);

			current_diagnostics = current_diagnostics
				.into_iter()
				.filter(|d| diagnostic_data_matches(d, current_scrubber, index))
				.collect();
			
			if current_diagnostics.len() == 0 {
				panic!("After filtering on index {} there are no diagnostics", index);
			} else if current_diagnostics.len() == 1 {
				break;
			}
			index += 1;

			if index >= current_diagnostics[0].len() {
				// can only scan to the last character
				break;
			}
		}

		convert_to_decimal(current_diagnostics[0])
	}
}

struct DiagnosticSummary {
	summary: Option<Vec<i32>>,
	count: i32,
}

impl DiagnosticSummary {
	fn new() -> DiagnosticSummary {
		DiagnosticSummary {
			summary: None,
			count: 0
		}
	}

	fn analyze_diagnostic(&mut self, diagnostic: &str) {
		if let None = self.summary {
			// first entry determines the size
			self.summary = Some(vec![0; diagnostic.len().try_into().unwrap()]);
		}
		let summary = self.summary.as_mut().unwrap();
		if diagnostic.len() != summary.len() {
			panic!("Entry index {} with length {} does not match the expected lengh of {}", self.count, diagnostic.len(), summary.len());
		}
		for (i, b) in diagnostic.bytes().enumerate() {
			match b {
				b'0' => {},
				b'1' => summary[i] += 1,
				_ => panic!("Entry '{}' has an invalid character at column {} - only 0 and 1 are accepted input", diagnostic, i),
			}
		}
		self.count += 1;
	}

	fn oxygen_rating(&self, index: usize) -> u8 {
		let sum = self.summary.as_ref().unwrap()[index];
		if sum as f64 >= (self.count as f64 / 2.0) {
			b'1'
		} else {
			b'0'
		}
	}

	fn scrubber_rating(&self, index: usize) -> u8 {
		let sum = self.summary.as_ref().unwrap()[index];
		if (sum as f64) < (self.count as f64 / 2.0) {
			b'1'
		} else {
			b'0'
		}
	}
}

fn diagnostic_data_matches(diagnostic: &str, bit: u8, index: usize) -> bool {
	let diagnostic_bit = diagnostic.as_bytes()[index];
	// let matches = diagnostic_bit == bit;
	// println!("Comparing {} at index {} ({}) to bit {} - {}", diagnostic, index, diagnostic_bit, bit, matches);
	diagnostic_bit == bit
}

fn convert_to_decimal(diagnostic: &str) -> i32 {
	i32::from_str_radix(&diagnostic, 2).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example_oxygen() {
		let mut life_support = LifeSupport::new();
		life_support.load_diagnostic("00100");
		life_support.load_diagnostic("11110");
		life_support.load_diagnostic("10110");
		life_support.load_diagnostic("10111");
		life_support.load_diagnostic("10101");
		life_support.load_diagnostic("01111");
		life_support.load_diagnostic("00111");
		life_support.load_diagnostic("11100");
		life_support.load_diagnostic("10000");
		life_support.load_diagnostic("11001");
		life_support.load_diagnostic("00100");
		life_support.load_diagnostic("01010");

		let oxygen_rating = life_support.oxygen_rating();
		assert_eq!(23, oxygen_rating);
	}

	#[test]
	fn test_example_scrubber() {
		let mut life_support = LifeSupport::new();
		life_support.load_diagnostic("00100");
		life_support.load_diagnostic("11110");
		life_support.load_diagnostic("10110");
		life_support.load_diagnostic("10111");
		life_support.load_diagnostic("10101");
		life_support.load_diagnostic("01111");
		life_support.load_diagnostic("00111");
		life_support.load_diagnostic("11100");
		life_support.load_diagnostic("10000");
		life_support.load_diagnostic("11001");
		life_support.load_diagnostic("00100");
		life_support.load_diagnostic("01010");

		let scrubber_rating = life_support.scrubber_rating();
		assert_eq!(10, scrubber_rating);
	}
}