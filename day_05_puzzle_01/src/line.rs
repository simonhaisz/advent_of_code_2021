use regex::Regex;
use std::cmp;
use std::hash::Hash;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

#[derive(Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        Line {
            start,
            end,
        }
    }

    pub fn from(input: &str) -> Line {
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new(r"^\s*(?P<start_x>\d+)\s*,\s*(?P<start_y>\d+)\s*->\s*(?P<end_x>\d+)\s*,\s*(?P<end_y>\d+)\s*$").unwrap();
        }

        let captures = LINE_REGEX.captures(input);
        if let Some(captures) = captures {
            let start = Point::new(
                u32::from_str_radix(&captures["start_x"], 10).unwrap(),
                u32::from_str_radix(&captures["start_y"], 10).unwrap()
            );
            let end = Point::new(
                u32::from_str_radix(&captures["end_x"], 10).unwrap(),
                u32::from_str_radix(&captures["end_y"], 10).unwrap()
            );
            Line::new(start, end)
        } else {
            panic!("")
        }
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn end(&self) -> &Point {
        &self.end
    }

    pub fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn min_x(&self) -> u32 {
        cmp::min(self.start().x, self.end().x)
    }

    pub fn max_x(&self) -> u32 {
        cmp::max(self.start().x(), self.end().x)
    }

    pub fn min_y(&self) -> u32 {
        cmp::min(self.start().y(), self.end().y())
    }

    pub fn max_y(&self) -> u32 {
        cmp::max(self.start().y(), self.end().y())
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];

        if self.horizontal() {
            let y = self.start().y();
            for x in self.min_x()..=self.max_x() {
                points.push(Point::new(x, y));
            }
        } else if self.vertical() {
            let x = self.start().x();
            for y in self.min_y()..=self.max_y() {
                points.push(Point::new(x, y));
            }
        }

        points
    }
}

pub fn intersections_optimized(a: &Line, b: &Line) -> Vec<Point> {
    let mut points = vec!();

    if (!a.horizontal() && !a.vertical()) || (!b.horizontal() && !b.vertical()) {
        panic!("Expected lines to be either horizontal or vertical in order to determine intersections\nline a:{:?}\nline b:{:?}", a, b);
    }

    if a.horizontal() && b.horizontal() {
        // if both horizontal then they only intersect if matching y axis
        if a.start().y() == b.start().y() {
            let a_min_x = a.min_x();
            let a_max_x = a.max_x();

            let b_min_x = b.min_x();
            let b_max_x = b.max_x();

            let intersection_min = cmp::max(a_min_x, b_min_x);
            let intersection_max = cmp::min(a_max_x, b_max_x);

            if intersection_min <= intersection_max {
                for x in intersection_min..=intersection_max {
                    points.push(Point::new(x, a.start().y()));
                }
            }
        }
    } else if a.vertical() && b.vertical() {
        // if both vertical then they only intersect matching x axis
        if a.start().x() == b.start().x() {
            let a_min_y = a.min_y();
            let a_max_y = a.max_y();

            let b_min_y = b.min_y();
            let b_max_y = b.max_y();

            let intersection_min = cmp::max(a_min_y, b_min_y);
            let intersection_max = cmp::min(a_max_y, b_max_y);

            if intersection_min <= intersection_max {
                for y in intersection_min..=intersection_max {
                    points.push(Point::new(a.start().x(), y));
                }
            }
        }
    } else {
        // one horizontal and one vertical so can only intersect at one point
        let a_min_y = a.min_y();
        let a_max_y = a.max_y();

        let b_min_y = b.min_y();
        let b_max_y = b.max_y();

        let y_overlap = if a_min_y == a_max_y && b_min_y <= a_min_y && b_max_y >= a_min_y {
            true
        } else if b_min_y == b_max_y && a_min_y <= b_min_y && a_max_y >= b_min_y {
            true
        } else {
            false
        };

        let a_min_x = a.min_x();
        let a_max_x = a.max_x();

        let b_min_x = b.min_x();
        let b_max_x = b.max_x();

        let x_overlap = if a_min_x == a_max_x && b_min_x <= a_min_x && b_max_x >= a_min_x {
            true
        } else if b_min_x == b_max_x && a_min_x <= b_min_x && a_max_x >= b_min_x {
            true
        } else {
            false
        };

        if x_overlap && y_overlap {
            let x = if a_min_x == a_max_x {
                a_min_x
            } else if b_min_x == b_max_x {
                b_min_x
            } else {
                panic!("One ofthe lines should be vertical\nline a:{:?}\nline b:{:?}", a, b)
            };

            let y = if a_min_y == a_max_y {
                a_min_y
            } else if b_min_y == b_min_y {
                b_min_y
            } else {
                panic!("One ofthe lines should be horizotal\nline a:{:?}\nline b:{:?}", a, b)
            };

            points.push(Point::new(x, y));
        }
    }

    points
}

pub fn intersections_unoptimized(a: &Line, b: &Line) -> Vec<Point> {
    let mut points = vec![];

    let a_points = a.points();
    let b_points = b.points();

    for a_point in a_points.iter() {
        for b_point in b_points.iter() {
            if a_point == b_point {
                points.push(Point::new(a_point.x(), a_point.y()));
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let line = Line::from("1,5->3,7");
        assert_eq!(Line::new(Point::new(1, 5), Point::new(3, 7)), line);
    }

    #[test]
    fn test_line_whitespace() {
        let line = Line::from(" 2,\t9 ->   13\t,\n7  ");
        assert_eq!(Line::new(Point::new(2, 9), Point::new(13, 7)), line);
    }

    #[test]
    fn test_horizontal() {
        let horizontal = Line::new(Point::new(0, 5), Point::new(12, 5));
        assert_eq!(true, horizontal.horizontal());

        let vertical = Line::new(Point::new(3, 2), Point::new(3, 21));
        assert_eq!(false, vertical.horizontal());

        let diagonal = Line::new(Point::new(0, 5), Point::new(5, 10));
        assert_eq!(false, diagonal.horizontal());
    }

    #[test]
    fn test_vertical() {
        let horizontal = Line::new(Point::new(0, 5), Point::new(12, 5));
        assert_eq!(false, horizontal.vertical());

        let vertical = Line::new(Point::new(3, 2), Point::new(3, 21));
        assert_eq!(true, vertical.vertical());

        let diagonal = Line::new(Point::new(0, 5), Point::new(5, 10));
        assert_eq!(false, diagonal.vertical());
    }

    #[test]
    fn test_no_intersections_parallel() {
        let line_a = Line::new(Point::new(0, 0), Point::new(0, 5));

        let line_b = Line::new(Point::new(1, 0), Point::new(1, 5));

        let points = intersections_unoptimized(&line_a, &line_b);

        assert_eq!(vec![] as Vec<Point>, points);

        let line_a = Line::new(Point::new(0, 0), Point::new(10, 0));

        let line_b = Line::new(Point::new(0, 2), Point::new(20, 2));

        let points = intersections_optimized(&line_a, &line_b);

        assert_eq!(vec![] as Vec<Point>, points);
    }

    #[test]
    fn test_no_intersections_perpendicular() {
        let line_a = Line::new(Point::new(0, 0), Point::new(0, 5));

        let line_b = Line::new(Point::new(1, 0), Point::new(5, 0));

        let points = intersections_unoptimized(&line_a, &line_b);

        assert_eq!(vec![] as Vec<Point>, points);

        let line_a = Line::new(Point::new(0, 0), Point::new(10, 0));

        let line_b = Line::new(Point::new(0, 2), Point::new(0, 20));

        let points = intersections_optimized(&line_a, &line_b);

        assert_eq!(vec![] as Vec<Point>, points);
    }

    #[test]
    fn test_intersections_parallel() {
        let line_a = Line::new(Point::new(0, 0), Point::new(0, 5));

        let line_b = Line::new(Point::new(0, 4), Point::new(0, 7));

        let points = intersections_unoptimized(&line_a, &line_b);

        assert_eq!(vec![Point::new(0, 4), Point::new(0, 5)], points);

        let line_a = Line::new(Point::new(0, 0), Point::new(10, 0));

        let line_b = Line::new(Point::new(10, 0), Point::new(20, 0));

        let points = intersections_optimized(&line_a, &line_b);

        assert_eq!(vec![Point::new(10, 0)], points);
    }

    #[test]
    fn test_intersections_perpendicular() {
        let line_a = Line::new(Point::new(0, 0), Point::new(0, 5));

        let line_b = Line::new(Point::new(0, 2), Point::new(5, 2));

        let points = intersections_optimized(&line_a, &line_b);

        assert_eq!(vec![Point::new(0, 2)], points);

        let line_a = Line::new(Point::new(0, 3), Point::new(10, 3));

        let line_b = Line::new(Point::new(2, 1), Point::new(2, 20));

        let points = intersections_optimized(&line_a, &line_b);

        assert_eq!(vec![Point::new(2, 3)] as Vec<Point>, points);
    }
}