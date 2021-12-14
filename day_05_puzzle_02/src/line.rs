use regex::Regex;
use std::cmp;
use std::hash::Hash;
use crate::geometry;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
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
                i32::from_str_radix(&captures["start_x"], 10).unwrap(),
                i32::from_str_radix(&captures["start_y"], 10).unwrap()
            );
            let end = Point::new(
                i32::from_str_radix(&captures["end_x"], 10).unwrap(),
                i32::from_str_radix(&captures["end_y"], 10).unwrap()
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

    pub fn valid(&self) -> bool {
        self.horizontal() || self.vertical() || self.diagonal()
    }

    pub fn diagonal(&self) -> bool {
        let delta_x = self.max_x() - self.min_x();
        let delta_y = self.max_y() - self.min_y();

        delta_x == delta_y
    }

    pub fn slope_x(&self) -> i32 {
        // don't care about 'direction' of the line (start and end)
        // only care about how x-axis changes as the y-axis changes
        if self.start().y() < self.end().y() {
            self.end().x() - self.start().x()
        } else if self.end().y() < self.start().y() {
            self.start().x() - self.end().x()
        } else {
            panic!("Slope of x is infinite because it is a horizontal line")
        }
    }

    pub fn slope_y(&self) -> i32 {
        // don't care about 'direction' of the line (start and end)
        // only care about how y-axis changes as the x-axis changes
        if self.start().x() < self.end().x() {
            self.end().y() - self.start().y()
        } else if self.end().x() < self.start().x() {
            self.start().y() - self.end().y()
        } else {
            panic!("Slope of y is infinite because it is a vertical line")
        }
    }

    pub fn min_x(&self) -> i32 {
        cmp::min(self.start().x, self.end().x)
    }

    pub fn max_x(&self) -> i32 {
        cmp::max(self.start().x(), self.end().x)
    }

    pub fn min_y(&self) -> i32 {
        cmp::min(self.start().y(), self.end().y())
    }

    pub fn max_y(&self) -> i32 {
        cmp::max(self.start().y(), self.end().y())
    }

    pub fn x_dimension(&self) -> (i32, i32) {
        (self.min_x(), self.max_x())
    }

    pub fn y_dimension(&self) -> (i32, i32) {
        (self.min_y(), self.max_y())
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
        } else if self.diagonal() {
            let delta_x = if self.end().x() == self.start().x() { 0 } else if self.end().x() > self.start().x() { 1 } else { -1 };
            let delta_y = if self.end().y() == self.start().y() { 0 } else if self.end().y() > self.start().y() { 1 } else { -1 };
            let mut x = self.start().x();
            let mut y = self.start().y();
            loop {
                points.push(Point::new(x, y));
                if x == self.end().x() && y >= self.end().y() {
                    break;
                }
                x += delta_x;
                y += delta_y;
            }
        } else {
            todo!()
        }

        points
    }
}

#[allow(dead_code)]
fn intersections_optimized(a: &Line, b: &Line) -> Vec<Point> {
    if !a.valid() || !b.valid() {
        panic!("Expected lines to be either horizontal, vertical, or diagonal in order to determine intersections\nline a:{:?}\nline b:{:?}", a, b);
    }

    let mut points = vec!();

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

pub fn intersections_specialized(a: &Line, b: &Line) -> Vec<Point> {
    if !a.valid() || !b.valid() {
        panic!("Expected lines to be either horizontal, vertical, or diagonal in order to determine intersections\nline a:{:?}\nline b:{:?}", a, b);
    }

    let mut points = vec![];

    if parallel(a, b) {
        if a.horizontal() && b.horizontal() {
            if a.start().y() == b.start().y() {
                let a_flat = a.x_dimension();
                let b_flat = b.x_dimension();
                if let Some(overlap_range) = geometry::overlap_range(a_flat.0, a_flat.1, b_flat.0, b_flat.1) {
                    for x in overlap_range.0..=overlap_range.1 {
                        points.push(Point::new(x, a.start().y()));
                    }
                }
            }
        } else if a.vertical() && b.vertical() {
            if a.start().x() == b.start().x() {
                let a_flat = a.y_dimension();
                let b_flat = b.y_dimension();
                if let Some(overlap_range) = geometry::overlap_range(a_flat.0, a_flat.1, b_flat.0, b_flat.1) {
                    for y in overlap_range.0..=overlap_range.1 {
                        points.push(Point::new(a.start().x(), y));
                    }
                }
            }
        } else if a.diagonal() && b.diagonal() {
            if a.start().x() - a.start().y == b.start().x - b.start().y() {
                let a_x_flat = a.x_dimension();
                let b_x_flat = b.x_dimension();

                if let Some(overlap_x_range) = geometry::overlap_range(a_x_flat.0, a_x_flat.1, b_x_flat.0, b_x_flat.1) {
                    let a_y_flat = a.y_dimension();
                    let b_y_flat = b.y_dimension();

                    if let Some(overlap_y_range) = geometry::overlap_range(a_y_flat.0, a_y_flat.1, b_y_flat.0, b_y_flat.1) {

                        // ranges are sorted independently, need to find out the order for each
                        if a.slope_y() > 0 {
                            let mut x = overlap_x_range.0;
                            let mut y = overlap_y_range.0;
                            // diagonals are all at 45-degree so the amount of overlap across x and y has to match
                            let delta = overlap_x_range.1 - overlap_x_range.0;
                            for _ in 0..delta {
                                points.push(Point::new(x, y));
                                x += 1;
                                y += 1;
                            }
                        } else {
                            let mut x = overlap_x_range.0;
                            let mut y = overlap_y_range.1;
                            // diagonals are all at 45-degree so the amount of overlap across x and y has to match
                            let delta = overlap_x_range.1 - overlap_x_range.0;
                            for _ in 0..delta {
                                points.push(Point::new(x, y));
                                x += 1;
                                y -= 1;
                            }
                        }
                    }
                }
            }
        } else {
            panic!("Lines are considered parallel but do not match alignment\nline a: {:?}\nline b: {:?}", a, b);
        }
    } else {
        let a_x_flat = a.x_dimension();
        let b_x_flat = b.x_dimension();

        if let Some(overlap_x_range) = geometry::overlap_range(a_x_flat.0, a_x_flat.1, b_x_flat.0, b_x_flat.1) {
            let a_y_flat = a.y_dimension();
            let b_y_flat = b.y_dimension();

            if let Some(overlap_y_range) = geometry::overlap_range(a_y_flat.0, a_y_flat.1, b_y_flat.0, b_y_flat.1) {
                if (a.horizontal() || a.vertical()) && (b.horizontal() || b.vertical()) {
                    // with overlapping vertical and horizontal (or horizontal and vertical) lines they can only overlap where they intersect, a single point
                    if overlap_x_range.0 != overlap_x_range.1 || overlap_y_range.0 != overlap_y_range.1 {
                        panic!("Perpendicular lines can only overlap at one point\nx range: {:?}\ny range: {:?}", overlap_x_range, overlap_y_range);
                    }
                    points.push(Point::new(overlap_x_range.0, overlap_y_range.0));
                } else {
                    let a_points = a.points();
                    let b_points = b.points();
                
                    for a_point in a_points.iter() {
                        for b_point in b_points.iter() {
                            if a_point == b_point {
                                points.push(Point::new(a_point.x(), a_point.y()));
                            }
                        }
                    }
                }
            }
        } 
    }

    points
}

// Keep alternate implementation
#[allow(dead_code)]
fn intersections_unoptimized(a: &Line, b: &Line) -> Vec<Point> {
    if !a.valid() || !b.valid() {
        panic!("Expected lines to be either horizontal, vertical, or diagonal in order to determine intersections\nline a:{:?}\nline b:{:?}", a, b);
    }

    let mut points = vec![];

    if geometry::overlap(a.min_x(), a.max_x(), b.min_x(), b.max_x()) && geometry::overlap(a.min_y(), a.max_y(), b.min_y(), b.max_y()) {
        let a_points = a.points();
        let b_points = b.points();
    
        for a_point in a_points.iter() {
            for b_point in b_points.iter() {
                if a_point == b_point {
                    points.push(Point::new(a_point.x(), a_point.y()));
                }
            }
        }
    }

    points
}

fn parallel(a: &Line, b: &Line) -> bool {
    if a.horizontal() && b.horizontal() {
        true
    } else if a.vertical() && b.vertical() {
        true
    } else if a.diagonal() && b.diagonal() {
        a.slope_x() == b.slope_x() && a.slope_y() == b.slope_y()
    } else {
        false
    }
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
    fn test_diagonal() {
        let horizontal = Line::new(Point::new(0, 5), Point::new(12, 5));
        assert_eq!(false, horizontal.diagonal());

        let vertical = Line::new(Point::new(3, 2), Point::new(3, 21));
        assert_eq!(false, vertical.diagonal());

        let diagonal = Line::new(Point::new(0, 5), Point::new(5, 10));
        assert_eq!(true, diagonal.diagonal());
    }

    #[test]
    fn test_parallel() {
        let horizontal_a = Line::new(Point::new(0, 0), Point::new(10, 0));
        let horizontal_b = Line::new(Point::new(0, 5), Point::new(10, 5));

        assert_eq!(true, parallel(&horizontal_a, &horizontal_b));

        let vertical_a = Line::new(Point::new(0, 0), Point::new(0, 10));
        let vertical_b = Line::new(Point::new(5, 0), Point::new(5, 10));

        assert_eq!(true, parallel(&vertical_a, &vertical_b));

        let diagonal_forward_a = Line::new(Point::new(0, 0), Point::new(10, 10));
        let diagonal_forward_b = Line::new(Point::new(5, 0), Point::new(15, 10));

        assert_eq!(true, parallel(&diagonal_forward_a, &diagonal_forward_b));

        let diagonal_backward_a = Line::new(Point::new(10, 0), Point::new(0, 10));
        let diagonal_backward_b = Line::new(Point::new(15, 0), Point::new(5, 10));

        assert_eq!(true, parallel(&diagonal_backward_a, &diagonal_backward_b));

        assert_eq!(false, parallel(&horizontal_a, &vertical_b));
        assert_eq!(false, parallel(&horizontal_a, &diagonal_forward_b));
        assert_eq!(false, parallel(&horizontal_a, &diagonal_backward_b));

        assert_eq!(false, parallel(&vertical_a, &horizontal_b));
        assert_eq!(false, parallel(&vertical_a, &diagonal_forward_b));
        assert_eq!(false, parallel(&vertical_a, &diagonal_backward_b));

        assert_eq!(false, parallel(&diagonal_forward_a, &horizontal_b));
        assert_eq!(false, parallel(&diagonal_forward_a, &vertical_b));
        assert_eq!(false, parallel(&diagonal_forward_a, &diagonal_backward_b));

        assert_eq!(false, parallel(&diagonal_backward_a, &horizontal_b));
        assert_eq!(false, parallel(&diagonal_backward_a, &vertical_b));
        assert_eq!(false, parallel(&diagonal_backward_a, &diagonal_forward_b));
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