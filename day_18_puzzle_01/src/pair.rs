use std::fmt::{self, Display};

pub enum Element {
	Number(u32),
	Pair(Box<Element>, Box<Element>),
}

struct ExplodeSet<'a> {
	left: Option<&'a Element>,
	center: &'a Element,
	right: Option<&'a Element>,
}

impl Element {

	pub fn new_number(n: u32) -> Element {
		Element::Number(n)
	}

	pub fn new_pair(left: Element, right: Element) -> Element {
		Element::Pair(Box::new(left), Box::new(right))
	}

	fn is_number(&self) -> bool {
		if let Element::Number(_) = self {
			true
		} else {
			false
		}
	}

	fn is_pair(&self) -> bool {
		if let Element::Pair(_, _) = self {
			true
		} else {
			false
		}
	}

	fn number(&self) -> u32 {
		if let Element::Number(n) = self {
			
		} else {
			panic!("Cannot get a number from a pair")
		}
	}

	fn add_number(&mut self, other: &Element) {
		self = &mut Element::new_number(self.number() + other.number());
	}
}

impl Display for Element {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Element::Number(n) => write!(f, "{}", n)?,
			Element::Pair(left, right) => write!(f, "[{}, {}]", left, right)?,
		}

        Ok(())
    }
}

fn reduce(element: &mut Element, search_level: u32) {
	if let Element::Pair(_, _) = element {
		if explode(search_level + 1, element) {

		} else {

		}
	}
}

fn explode(search_level: u32, element: &mut Element, parent_left: Option<&mut Element>, parent_right: Option<&mut Element>) -> bool {
	if let Element::Pair(left, right) = element {
		if search_level == 4 {
			if !left.is_number() || !right.is_number() {
				panic!("Exploding pairs should always consist of numbers")
			}
			element = &mut Element::new_number(0);
			return true;
		}
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_display() {
		let root = Element::new_pair(Element::new_number(7), Element::new_pair(Element::new_number(4), Element::new_number(3)));
		let output = root.to_string();
		assert_eq!("[7, [4, 3]]", output);
	}
}