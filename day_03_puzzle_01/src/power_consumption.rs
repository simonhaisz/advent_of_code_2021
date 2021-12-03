pub struct PowerConsumption {
	diagnostic_summary: Option<Vec<i32>>,
	diagnostic_entry_count: i32,
}

impl PowerConsumption {
	pub fn new() -> PowerConsumption {
		PowerConsumption {
			diagnostic_summary: None,
			diagnostic_entry_count: 0,
		}
	}

	pub fn analyze_entry(&mut self, entry: &str) {
		if let None = self.diagnostic_summary {
			// first entry determines the size
			self.diagnostic_summary = Some(vec![0; entry.len().try_into().unwrap()]);
		}
		let summary = self.diagnostic_summary.as_mut().unwrap();
		if entry.len() != summary.len() {
			panic!("Entry index {} with length {} does not match the expected lengh of {}", self.diagnostic_entry_count, entry.len(), summary.len());
		}
		for (i, b) in entry.bytes().enumerate() {
			match b {
				b'0' => {},
				b'1' => summary[i] += 1,
				_ => panic!("Entry '{}' has an invalid character at column {} - only 0 and 1 are accepted input", entry, i),
			}
		}
		self.diagnostic_entry_count += 1;
	}

	pub fn gamme_rate(&self) -> i32 {
		let comparison = |c: i32| -> bool {
			c as f64 > (self.diagnostic_entry_count as f64 / 2.0)
		};
		self.rate(comparison)
	}

	pub fn epsilon_rate(&self) -> i32 {
		let comparison = |c: i32| -> bool {
			c as f64 <= (self.diagnostic_entry_count as f64 / 2.0)
		};
		self.rate(comparison)
	}

	fn rate<F>(&self, comparison: F) -> i32
		where F: Fn(i32) -> bool {
		let encoded_value = self.diagnostic_summary.as_ref().unwrap().into_iter().map(|&c| if comparison(c) { "1" } else { "0" }).collect::<String>();
		i32::from_str_radix(&encoded_value, 2).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_gamma() {
		let mut power = PowerConsumption::new();
		power.analyze_entry("000");
		power.analyze_entry("010");
		power.analyze_entry("110");
		let gamme_rate = power.gamme_rate();
		// 010 => 2
		assert_eq!(2, gamme_rate);
	}

	#[test]
	fn test_epsilon() {
		let mut power = PowerConsumption::new();
		power.analyze_entry("000");
		power.analyze_entry("010");
		power.analyze_entry("110");
		let epsilon_rate = power.epsilon_rate();
		// 101 => 5
		assert_eq!(5, epsilon_rate);
	}
}