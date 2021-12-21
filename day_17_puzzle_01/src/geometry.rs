use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y,
        }
    }

    pub fn origin() -> Position {
        Position::new(0, 0)
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

pub struct Rectangle {
    top_left: Position,
    top_right: Position,
    bottom_left: Position,
    bottom_right: Position,
}

impl Rectangle {
    pub fn new(start_x: i32, end_x: i32, start_y: i32, end_y: i32) -> Rectangle {
        if start_x >= end_x {
            panic!("Start-x {} is expected to be smaller than end-x {}", start_x, end_x)
        }
        if start_y >= end_y {
            panic!("Start-y {} is expected to be smaller than end-y {}", start_y, end_y)
        }
        Rectangle {
            top_left: Position::new(start_x, end_y),
            top_right: Position::new(end_x, end_y),
            bottom_left: Position::new(start_x, start_y),
            bottom_right: Position::new(end_x, start_y),
        }
    }

    pub fn from(input: &str) -> Rectangle {
        lazy_static! {
            static ref AREA_REGEX: Regex = Regex::new(r"x=(?P<start_x>-?\d+)\.\.(?P<end_x>-?\d+),\s*y=(?P<start_y>-?\d+)\.\.(?P<end_y>-?\d+)$").unwrap();
        }

        let captures = AREA_REGEX.captures(input);
        if let Some(captures) = captures {
            Rectangle::new(
                i32::from_str_radix(&captures["start_x"], 10).unwrap(),
                i32::from_str_radix(&captures["end_x"], 10).unwrap(),
                i32::from_str_radix(&captures["start_y"], 10).unwrap(),
                i32::from_str_radix(&captures["end_y"], 10).unwrap()
            )
        } else {
            panic!("Failed to parse area from '{}'", input)
        }
    }

    pub fn short(&self, pos: &Position) -> bool {
        (pos.x < self.top_left.x && pos.y > self.top_left.y) ||
        (pos.x < self.top_left.x && pos.y >= self.bottom_left.y) ||
        (pos.y > self.top_right.y && pos.x <= self.top_right.x)
    }

    pub fn within(&self, pos: &Position) -> bool {
        pos.x >= self.top_left.x &&
        pos.x <= self.bottom_right.x &&
        pos.y <= self.top_right.y &&
        pos.y >= self.bottom_left.y
    }

    pub fn far(&self, pos: &Position) -> bool {
        pos.x > self.top_right.x ||
        pos.y < self.bottom_right.y
    }

    pub fn horizontal_range(&self) -> (i32, i32) {
        (self.top_left.x(), self.top_right.x())
    }

    pub fn vertical_range(&self) -> (i32, i32) {
        (self.top_left.y(), self.bottom_left.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within() {
        let rect = Rectangle::new(5, 10, -20, -10);

        let center = Position::new(7, -15);
        assert_eq!(true, rect.within(&center));

        let corner = Position::new(5, -20);
        assert_eq!(true, rect.within(&corner));

        let side = Position::new(10, -12);
        assert_eq!(true, rect.within(&side));
    }

    #[test]
    fn test_without() {
        let rect = Rectangle::new(5, 10, -20, -10);

        let top_left = Position::new(2, -8);
        assert_eq!(false, rect.within(&top_left));
        assert_eq!(true, rect.short(&top_left));
        assert_eq!(false, rect.far(&top_left));

        let bottom_right = Position::new(17, -23);
        assert_eq!(false, rect.within(&bottom_right));
        assert_eq!(false, rect.short(&bottom_right));
        assert_eq!(true, rect.far(&bottom_right));

        let to_the_left_to_the_left = Position::new(4, -15);
        assert_eq!(false, rect.within(&to_the_left_to_the_left));
        assert_eq!(true, rect.short(&to_the_left_to_the_left));
        assert_eq!(false, rect.far(&to_the_left_to_the_left));

        let bottoms_away = Position::new(6, -27);
        assert_eq!(false, rect.within(&bottoms_away));
        assert_eq!(false, rect.short(&bottoms_away));
        assert_eq!(true, rect.far(&bottoms_away));
    }
}