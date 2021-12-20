fn convert_hex_char_to_binary(hex: char) -> &'static str {
	match hex {
		'0' => "0000",
		'1' => "0001",
		'2' => "0010",
		'3' => "0011",
		'4' => "0100",
		'5' => "0101",
		'6' => "0110",
		'7' => "0111",
		'8' => "1000",
		'9' => "1001",
		'A' => "1010",
		'B' => "1011",
		'C' => "1100",
		'D' => "1101",
		'E' => "1110",
		'F' => "1111",
		_ => panic!("Invalid hex value {}", hex)
	}
}

pub fn convert_hex_value_to_binary(hex_value: &str) -> String {
	let mut binary_value = String::new();

	for hex_code in hex_value.chars() {
		binary_value.push_str(convert_hex_char_to_binary(hex_code));
	}

	binary_value
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_convert() {
		assert_eq!("110100101111111000101000", convert_hex_value_to_binary(&"D2FE28").as_str());

		assert_eq!("00111000000000000110111101000101001010010001001000000000", convert_hex_value_to_binary(&"38006F45291200").as_str());

		assert_eq!("11101110000000001101010000001100100000100011000001100000", convert_hex_value_to_binary(&"EE00D40C823060").as_str());
	}
}