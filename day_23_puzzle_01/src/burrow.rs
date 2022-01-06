use std::collections::HashMap;
use regex::Regex;
use crate::amphipod::Amphipod;

#[derive(PartialEq, Eq, Hash)]
pub enum Position {
	Hallway(usize),
	Room(usize, usize),
}

pub struct Burrow {
	hallway: (usize, usize),
	room_count: usize,
	room_size: usize,
	amphipods: HashMap<Position, Amphipod>,
}

impl Burrow {
	fn new(hallway: (usize, usize), rooms: HashMap<usize,Vec<Amphipod>>) -> Burrow {
		let mut amphipods = HashMap::new();
		let room_count = rooms.len();
		let mut room_size: Option<usize> = None;
		for (room_location, room_amphipods) in rooms.into_iter() {
			if room_size.is_none() {
				room_size = Some(room_amphipods.len());
			} else if room_size.unwrap() != room_amphipods.len() {
				panic!("All rooms are expected to be the same size - expected {}, found {}", room_size.unwrap(), room_amphipods.len())
			}
			for (amphipod_location, amphipod) in room_amphipods.into_iter().enumerate() {
				amphipods.insert(Position::Room(room_location, amphipod_location), amphipod);
			}
		}
		Burrow {
			hallway,
			room_count,
			room_size: room_size.unwrap(),
			amphipods
		}
	}

	pub fn from(input: &str) -> Burrow {
		let mut hallway: Option<(usize, usize)> = None;
		let mut rooms = HashMap::new();
		// make assumptions about the burrow to simplify parsing

		for line in input.split("\n") {
			let contents = get_interior_content(line);
			if contents.len() > 0 {
				for content in contents.into_iter() {
					match content {
						InteriorSpace::Hallway(start, end) => {
							if hallway.is_none() {
								hallway = Some((start, end));
							} else {
								panic!("There can only be one...hallway")
							}
						},
						InteriorSpace::Room(location, amphipod) => {
							let room = rooms.entry(location).or_insert(vec![]);
							room.push(amphipod);
						},
					}
				}
			}
		}

		Burrow::new(hallway.unwrap(), rooms)
	}
}

enum InteriorSpace {
	Hallway(usize, usize),
	Room(usize, Amphipod),
}

fn get_interior_content(line: &str) -> Vec<InteriorSpace> {
	let mut content = vec![];

	if line.contains("\n") {
		panic!("Line should not contain any new lines - '{}'", line);
	}

	lazy_static! {
		static ref WALL_REGEX: Regex = Regex::new(r"^\s*(?P<wall>#+)\s*$").unwrap();
		static ref HALLWAY_REGEX: Regex = Regex::new(r"^#(?P<hallway>\.+)#$").unwrap();
		static ref ROOM_REGEX: Regex = Regex::new(r"\b(?P<room>[ABCD])\b").unwrap();
	}

	if WALL_REGEX.is_match(line) {
		// NOOP
	} else if HALLWAY_REGEX.is_match(line) {
		if let Some(hallway_capture) = HALLWAY_REGEX.captures(line) {
			let hallway_match = hallway_capture.get(1).unwrap();
			content.push(InteriorSpace::Hallway(hallway_match.start(), hallway_match.end()));
		} else {
			panic!("Matched against hallway regex but could not capture against regex - '{}'", line)
		}
	} else if ROOM_REGEX.is_match(line) {
		for room_capture in ROOM_REGEX.captures_iter(line) {
			let room = room_capture.get(1).unwrap();
			content.push(InteriorSpace::Room(room.start(), Amphipod::from(room.as_str())));
		}
	} else {
		panic!("Line does not match any known format - '{}'", line)
	}

	content
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn burrow_parse() {
		let burrow = Burrow::from("#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"
		);

		assert_eq!((1, 12), burrow.hallway);
		assert_eq!(4, burrow.room_count);
		assert_eq!(2, burrow.room_size);
		assert_eq!(8, burrow.amphipods.len());
	}
}