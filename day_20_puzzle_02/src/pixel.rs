use regex::Regex;
use std::fmt::{self, Display};

pub type PixelGrid = Vec<String>;

const DARK: char = '.';
const LIGHT: char = '#';

pub struct Image {
	pixels: PixelGrid,
	infinite_pixels: char,
}

impl Image {
	pub fn new(pixels: PixelGrid) -> Image {
		Image {
			pixels,
			infinite_pixels: DARK,
		}
	}

	pub fn size(&self) -> isize {
		self.pixels.len() as isize
	}

	fn decode_pixel(&self, x: isize, y: isize) -> u32 {
		let mut line = String::new();

		let lower_limit = 0;
		let upper_limit = self.size() - 1;
		
		for row in y - 1..=y + 1 {
			if row < lower_limit || row > upper_limit {
				for _ in 0..3 {
					line.push(self.infinite_pixels);
				}
			} else {
				for column in x - 1..=x + 1 {
					if column < lower_limit || column > upper_limit {
						line.push(self.infinite_pixels);
					} else {
						if let Some(p) = self.pixels[row as usize].chars().nth(column as usize) {
							line.push(p);
						} else {
							panic!("Failed to find a pixel as location ({}, {}) in a square of size {}", column, row, self.size())
						}
					}
				}
			}
		}

		decode(&line)
	}

	fn enhance_pixel(&self, enhancement: &str, x: isize, y: isize) -> char {
		let decoded = self.decode_pixel(x, y);
		enhancement.chars().nth(decoded as usize).unwrap()
	}

	pub fn enhance(&self, enhancement: &str) -> Image {
		let mut enhanced_pixels: PixelGrid = vec![];
		for row in -1..=self.size() {
			let mut line = String::new();
			for column in -1..=self.size() {
				line.push(self.enhance_pixel(enhancement, column, row));
			}
			enhanced_pixels.push(line);
		}

		let enhanced_infinite_pixels = if self.infinite_pixels == DARK {
			// all zeros means an index of 000000000
			enhancement.chars().nth(0).unwrap()
		} else {
			// all ones means an index of 111111111
			enhancement.chars().nth(511).unwrap()
		};

		Image {
			pixels: enhanced_pixels,
			infinite_pixels: enhanced_infinite_pixels,
		}
	}

	pub fn lit_pixel_count(&self) -> usize {
		let mut count = 0;
		for row in self.pixels.iter() {
			count += row.chars()
				.filter(|p| *p == '#')
				.count();
		}
		count
	}
}

impl Display for Image {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (index, row) in self.pixels.iter().enumerate() {
			if index > 0 {
				write!(f, "\n")?;
			}
			write!(f, "{}", row)?;
		}
		Ok(())
	}
}

pub fn decode(input: &str) -> u32 {
	lazy_static! {
		static ref PIXEL_FORMAT_REGEX: Regex = Regex::new(r"^[.#]+$").unwrap();
	}
	if !PIXEL_FORMAT_REGEX.is_match(input) {
		panic!("Invalid input '{}' - expected a series of . and # characters", input)
	}

	let binary = input
		.replace(DARK, "0")
		.replace(LIGHT, "1");
	
	u32::from_str_radix(&binary, 2).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	pub fn from(input: &str) -> Image {
		let pixels = input
			.split("\n")
			.map(|l| l.trim().to_string())
			.filter(|l| !l.is_empty())
			.collect();

		Image {
			pixels,
			infinite_pixels: DARK,
		}
	}

	#[test]
	#[should_panic(expected = "Invalid input 'this is not pixel data' - expected a series of . and # characters")]
	fn decode_invalid_content() {
		decode("this is not pixel data");
	}

	#[test]
	#[should_panic(expected = "Invalid input '' - expected a series of . and # characters")]
	fn decode_invalid_empty() {
		decode("");
	}

	#[test]
	fn decode_valid() {
		assert_eq!(0, decode("."));
		assert_eq!(0, decode("...."));
		assert_eq!(1, decode("...#"));
		assert_eq!(15, decode("####"));
	}

	#[test]
	fn image_decode_pixel() {
		let image = from("
#..#.
#....
##..#
..#..
..###
		");
		
		assert_eq!(18, image.decode_pixel(0, 0));
		assert_eq!(32, image.decode_pixel(4, 0));
		assert_eq!(34, image.decode_pixel(2, 2));
		assert_eq!(0, image.decode_pixel(0, 4));
		assert_eq!(48, image.decode_pixel(4, 4));

		assert_eq!(0, image.decode_pixel(-2, -2));
		assert_eq!(0, image.decode_pixel(6, 6));
	}

	#[test]
	fn image_enhance() {
		let image = from("
#..#.
#....
##..#
..#..
..###
		");

		assert_eq!(
"#..#.
#....
##..#
..#..
..###",
			image.to_string()
		);

		assert_eq!(10, image.lit_pixel_count());

		let enhancement = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

		let enhanced_image = image.enhance(enhancement);

		assert_eq!(
".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.",
			enhanced_image.to_string()
		);

		assert_eq!(24, enhanced_image.lit_pixel_count());

		let enhanced_enhanced_image = enhanced_image.enhance(enhancement);

		assert_eq!(
".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..",
			enhanced_enhanced_image.to_string()
		);

		assert_eq!(35, enhanced_enhanced_image.lit_pixel_count());
	}

	#[test]
	fn image_enhance_50() {
		let mut image = from("
#..#.
#....
##..#
..#..
..###
		");

		let enhancement = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

		for _ in 0..50 {
			image = image.enhance(enhancement);
		}

		assert_eq!(3351, image.lit_pixel_count());
	}
}