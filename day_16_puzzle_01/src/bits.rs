use crate::hex;
use crate::packet::{Packet, LiteralPacket, OperatorPacket};

fn convert_to_binary(hex_value: &str) -> String {
	let mut binary_value = String::new();

	for hex_code in hex_value.chars() {
		binary_value.push_str(hex::convert_to_binary(hex_code));
	}

	binary_value
}

pub fn extract_packet(packet: &str) -> (Packet, u32) {
	let id = get_id(&packet);
	match id {
		TYPE_LITERAL_VALUE => {
			let (literal, bits_read) = extract_literal_packet(&packet);
			(Packet::Literal(Box::new(literal)), bits_read)
		},
		_ => {
			let (operator, bits_read) = extract_operator_packet(&packet);
			(Packet::Operator(Box::new(operator)), bits_read)
		},
	}
}

fn extract_literal_packet(packet: &str) -> (LiteralPacket, u32) {
	let version = get_version(&packet);
	let (value, bits_read) = get_literal_value(&packet);
	(LiteralPacket::new(version, value), bits_read)
}

fn extract_operator_packet(packet: &str) -> (OperatorPacket, u32) {
	let version = get_version(&packet);
	let (sub_packets, bits_read) = get_sub_packets(&packet);
	(OperatorPacket::new(version, sub_packets), bits_read)

}

fn get_version(packet: &str) -> u32 {
	if packet.len() < 3 {
		panic!("packet is too small - found size {}", packet.len())
	}

	convert_to_integer(&packet[..3])
}

fn get_id(packet: &str) -> u32 {
	if packet.len() < 6 {
		panic!("packet is too small - found size {}", packet.len())
	}

	convert_to_integer(&packet[3..6])
}

fn get_literal_value(packet: &str) -> (u32, u32) {
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
		index += 5;
		// and the first bit is a 'is there more data' flag
		if &group[..1] == "0" {
			break;
		}
	}

	(convert_to_integer(&literal_value.as_str()), index as u32)
}

fn get_sub_packets(packet: &str) -> (Vec<Packet>, u32) {
	let id = get_id(&packet);
	if id == TYPE_LITERAL_VALUE {
		panic!("Expected type to be an operator value - found {}", id)
	}

	let length_type_id = convert_to_integer(&packet[6..7]);
	if length_type_id == LENGTH_TYPE_PACKETS_SIZE {
		let sub_packets_bit_length = convert_to_integer(&packet[7..22]);
		let mut sub_packets = vec![];
		let mut total_bits_read = 0;
		let mut index = 22;
		loop {
			let (packet, bits_read) = extract_packet(&packet[index..]);
			sub_packets.push(packet);
			index += bits_read as usize;
			total_bits_read += bits_read;
			if total_bits_read == sub_packets_bit_length {
				break;
			} else if total_bits_read > sub_packets_bit_length {
				panic!("Read more bits than expected while extracting sub-packets - expected {}, read {}", sub_packets_bit_length, total_bits_read)
			}
		}
		(sub_packets, index as u32)
	} else if length_type_id == LENGTH_TYPE_PACKETS_COUNT {
		let sub_packet_count = convert_to_integer(&packet[7..18]);
		let mut sub_packets = vec![];
		let mut index = 18;
		while sub_packets.len() < sub_packet_count as usize {
			let (packet, bits_read) = extract_packet(&packet[index..]);
			sub_packets.push(packet);
			index += bits_read as usize;
		}
		(sub_packets, index as u32)
	} else {
		panic!("Unknown length type id for operator packet - expected 0 or 1, found {}", length_type_id)
	}
}

fn convert_to_integer(binary_value: &str) -> u32 {
	u32::from_str_radix(&binary_value, 2).unwrap()
}

const TYPE_LITERAL_VALUE: u32 = 4;

const LENGTH_TYPE_PACKETS_SIZE: u32 = 0;

const LENGTH_TYPE_PACKETS_COUNT: u32 = 1;

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
		assert_eq!((2021, 21), get_literal_value("110100101111111000101000"));
	}

	#[test]
	fn test_operator_type_length() {
		let (sub_packets, bits_read) = get_sub_packets("00111000000000000110111101000101001010010001001000000000");
		assert_eq!(2, sub_packets.len());
		assert_eq!(49, bits_read);

		assert_eq!(
			Packet::Literal(Box::new(LiteralPacket::new(6, 10))),
			sub_packets[0]
		);

		assert_eq!(
			Packet::Literal(Box::new(LiteralPacket::new(2, 20))),
			sub_packets[1]
		);
	}

	#[test]
	fn test_operator_type_count() {
		let (sub_packets, bits_read) = get_sub_packets("11101110000000001101010000001100100000100011000001100000");
		assert_eq!(3, sub_packets.len());
		assert_eq!(51, bits_read);

		assert_eq!(
			Packet::Literal(Box::new(LiteralPacket::new(2, 1))),
			sub_packets[0]
		);

		assert_eq!(
			Packet::Literal(Box::new(LiteralPacket::new(4, 2))),
			sub_packets[1]
		);

		assert_eq!(
			Packet::Literal(Box::new(LiteralPacket::new(1, 3))),
			sub_packets[2]
		);
	}
}