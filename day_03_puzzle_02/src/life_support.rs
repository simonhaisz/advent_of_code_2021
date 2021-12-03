pub struct LifeSupport {
	size: usize,
	diagnostic_data: Vec<u32>,
}

impl LifeSupport {

	pub fn new() -> LifeSupport {
		LifeSupport {
			size: 0,
			diagnostic_data: vec!()
		}
	}

	pub fn set_size(&mut self, size: usize) {
		self.size = size;
	}

	pub fn load_diagnostic(&mut self, data: u32) {
		// now that we are using numbers assume the binary rep has enough digits
		self.diagnostic_data.push(data);
	}

	pub fn oxygen_rating(&self) -> u32 {
		self.compute_rating(|summary: &DiagnosticSummary| summary.oxygen_rating())
	}

	pub fn scrubber_rating(&self) -> u32 {
		self.compute_rating(|summary: &DiagnosticSummary| summary.scrubber_rating())
	}

	fn compute_rating<F>(&self, get_rating: F) -> u32 where F: Fn(&DiagnosticSummary) -> bool {
		let mut current_diagnostics = self.diagnostic_data.clone();

		let mut index = 0;
		let mut shift = self.size - 1;

		while current_diagnostics.len() > 0 {
			let process_diagnostic = |d: u32| -> bool {
				let shifted_data = d >> shift;
				shifted_data & 0b1 == 0b1
			};
			let mut current_summary = DiagnosticSummary::new();
			for diagnostic in current_diagnostics.iter() {
				current_summary.analyze_diagnostic(process_diagnostic(*diagnostic));
			}

			let current_rating = get_rating(&current_summary);

			current_diagnostics = current_diagnostics
				.into_iter()
				.filter(|d| process_diagnostic(*d) == current_rating)
				.collect();
			
			if current_diagnostics.len() == 0 {
				panic!("After filtering on index {} there are no diagnostics", index);
			} else if current_diagnostics.len() == 1 {
				break;
			}
			index += 1;
			shift -= 1;

			if index >= self.size {
				// can only scan to the last character
				break;
			}
		}

		current_diagnostics[0]
	}
}

struct DiagnosticSummary {
	sum: i32,
	count: i32,
}

impl DiagnosticSummary {
	fn new() -> DiagnosticSummary {
		DiagnosticSummary {
			sum: 0,
			count: 0
		}
	}

	fn analyze_diagnostic(&mut self, diagnostic_on_off: bool) {
		self.sum += if diagnostic_on_off { 1 } else { -1 };
		self.count += 1;
	}

	fn oxygen_rating(&self) -> bool {
		if self.sum >= 0 {
			true
		} else {
			false
		} 
	}

	fn scrubber_rating(&self) -> bool {
		if self.sum >= 0 {
			false
		} else {
			true
		}
	}
}

fn diagnostic_data_matches(diagnostic_on_off: bool, target_on_off: bool) -> bool {
	diagnostic_on_off == target_on_off
}

fn convert_to_decimal(diagnostic: &str) -> u32 {
	u32::from_str_radix(&diagnostic, 2).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example_oxygen() {
		let mut life_support = LifeSupport::new();
		life_support.set_size(5);
		life_support.load_diagnostic(convert_to_decimal("00100"));
		life_support.load_diagnostic(convert_to_decimal("11110"));
		life_support.load_diagnostic(convert_to_decimal("10110"));
		life_support.load_diagnostic(convert_to_decimal("10111"));
		life_support.load_diagnostic(convert_to_decimal("10101"));
		life_support.load_diagnostic(convert_to_decimal("01111"));
		life_support.load_diagnostic(convert_to_decimal("00111"));
		life_support.load_diagnostic(convert_to_decimal("11100"));
		life_support.load_diagnostic(convert_to_decimal("10000"));
		life_support.load_diagnostic(convert_to_decimal("11001"));
		life_support.load_diagnostic(convert_to_decimal("00100"));
		life_support.load_diagnostic(convert_to_decimal("01010"));

		let oxygen_rating = life_support.oxygen_rating();
		assert_eq!(23, oxygen_rating);
	}

	#[test]
	fn test_example_scrubber() {
		let mut life_support = LifeSupport::new();
		life_support.set_size(5);
		life_support.load_diagnostic(convert_to_decimal("00100"));
		life_support.load_diagnostic(convert_to_decimal("11110"));
		life_support.load_diagnostic(convert_to_decimal("10110"));
		life_support.load_diagnostic(convert_to_decimal("10111"));
		life_support.load_diagnostic(convert_to_decimal("10101"));
		life_support.load_diagnostic(convert_to_decimal("01111"));
		life_support.load_diagnostic(convert_to_decimal("00111"));
		life_support.load_diagnostic(convert_to_decimal("11100"));
		life_support.load_diagnostic(convert_to_decimal("10000"));
		life_support.load_diagnostic(convert_to_decimal("11001"));
		life_support.load_diagnostic(convert_to_decimal("00100"));
		life_support.load_diagnostic(convert_to_decimal("01010"));

		let scrubber_rating = life_support.scrubber_rating();
		assert_eq!(10, scrubber_rating);
	}
}