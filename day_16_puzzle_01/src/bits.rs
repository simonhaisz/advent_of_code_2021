use crate::hex;

pub fn convert_to_binary(hex_value: &str) -> String {
	let mut binary_value = String::new();

	for hex_code in hex_value.chars() {
		binary_value.push_str(hex::convert_to_binary(hex_code));
	}

	binary_value
}

pub fn get_version(packet: &str) -> u32 {
	if packet.len() < 3 {
		panic!("packet is too small - found size {}", packet.len())
	}

	convert_to_integer(&packet[..3])
}

pub fn get_id(packet: &str) -> u32 {
	if packet.len() < 6 {
		panic!("packet is too small - found size {}", packet.len())
	}

	convert_to_integer(&packet[3..6])
}

pub fn get_literal_value(packet: &str) -> u32 {
	let id = get_id(&packet);
	if id != TYPE_LITERAL_VALUE {
		panic!("Expect type {} for literal value - found {}", TYPE_LITERAL_VALUE, id)
	}

	let mut literal_value = String::new();
	let mut index = 6;

	loop {
		// data in kept in groups of size 5
		let group = &packet[index..index + 5];
		// where the actual data is the last 4 bits
		literal_value.push_str(&group[1..]);
		// and the first bit is a 'is there more data' flag
		if &group[..1] == "0" {
			break;
		}
		index += 5;
	}

	convert_to_integer(&literal_value.as_str())
}

pub fn get_sub_packets(packet: &str) -> Vec<&str> {
	let id = get_id(&packet);
	if id == TYPE_LITERAL_VALUE {
		panic!("Expected type to be an operator value - found {}", id)
	}

	let length_type_id = convert_to_integer(&packet[6..7]);
	if length_type_id == 0 {
		let sub_packets_bit_length = convert_to_integer(&packet[7..22]);
		let mut sub_packets = vec![];

		sub_packets
	} else if length_type_id == 1 {
		let mut sub_packets = vec![];

		sub_packets
	} else {
		panic!("")
	}
}

fn convert_to_integer(binary_value: &str) -> u32 {
	u32::from_str_radix(&binary_value, 2).unwrap()
}

static TYPE_LITERAL_VALUE: u32 = 4;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_convert() {
		assert_eq!("110100101111111000101000", convert_to_binary(&"D2FE28").as_str());

		assert_eq!("00111000000000000110111101000101001010010001001000000000", convert_to_binary(&"38006F45291200").as_str());

		assert_eq!("11101110000000001101010000001100100000100011000001100000", convert_to_binary(&"EE00D40C823060").as_str());
	}

	#[test]
	fn test_convert_to_integer() {
		assert_eq!(0, convert_to_integer("0000"));

		assert_eq!(5, convert_to_integer("101"));

		assert_eq!(13, convert_to_integer("1101"));
	}

	#[test]
	fn test_version() {
		assert_eq!(6, get_version("11000"));
		assert_eq!(1, get_version("001111111"));
		assert_eq!(7, get_version("11110101101010101101"));
	}

	#[test]
	fn test_id() {
		assert_eq!(4, get_id("000100"));
		assert_eq!(6, get_id("111110010111010001"));
		assert_eq!(3, get_id("10101111001010"));
	}

	#[test]
	fn test_literal_value() {
		assert_eq!(2021, get_literal_value("110100101111111000101000"));
	}
}