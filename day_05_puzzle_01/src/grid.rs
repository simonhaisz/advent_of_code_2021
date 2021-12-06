use std::collections::HashSet;
use crate::line::{self, Line, Point};

pub struct Grid {
    lines: Vec<Line>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            lines: vec![],
        }
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn overlaps(&self) -> HashSet<Point> {
        let mut overlaps = HashSet::new();

        let lines = self.lines.iter().collect::<Vec<&Line>>();

        let mut compare_counter = 0;
        for outer in 0..(lines.len()-1) {
            for inner in (outer+1)..lines.len() {
                let a = &lines[outer];
                let b = &lines[inner];
                compare_counter += 1;
                for p in line::intersections_unoptimized(a, b).into_iter() {
                    overlaps.insert(p);
                }
            }
        }

        let expected_comparisons = lines.len() * (lines.len() - 1) / 2;

        println!("Compared {} pairs of lines together (expected {}) from a total of {}", compare_counter, expected_comparisons, lines.len());

        overlaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps_square() {
        let mut grid = Grid::new();
        grid.add_line(Line::new(Point::new(1, 1), Point::new(1, 10)));
        grid.add_line(Line::new(Point::new(1, 1), Point::new(10, 1)));
        grid.add_line(Line::new(Point::new(1, 10), Point::new(10, 10)));
        grid.add_line(Line::new(Point::new(10, 10), Point::new(10, 1)));

        let overlaps = grid.overlaps();
        assert_eq!(4, overlaps.len());
        assert_eq!(true, overlaps.contains(&Point::new(1, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(1, 10)));
        assert_eq!(true, overlaps.contains(&Point::new(10, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(10, 10)));
    }

    #[test]
    fn test_overlaps_thicc_line() {
        let mut grid = Grid::new();
        grid.add_line(Line::new(Point::new(1,1), Point::new(10, 1)));
        grid.add_line(Line::new(Point::new(9,1), Point::new(2, 1)));
        grid.add_line(Line::new(Point::new(5,1), Point::new(7, 1)));
        grid.add_line(Line::new(Point::new(7,1), Point::new(6, 1)));
        grid.add_line(Line::new(Point::new(6,1), Point::new(6, 1)));

        let overlaps = grid.overlaps();
        assert_eq!(8, overlaps.len());
        assert_eq!(true, overlaps.contains(&Point::new(2, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(3, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(4, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(5, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(6, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(7, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(8, 1)));
        assert_eq!(true, overlaps.contains(&Point::new(9, 1)));
    }

    #[test]
    fn test_sample() {
        let mut grid = Grid::new();
        let input = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
        ";
        let lines = input.split("\n").filter(|l| !l.trim().is_empty()).collect::<Vec<&str>>();

        for line in lines.iter() {
            let l = Line::from(line);
            if l.horizontal() || l.vertical() {
                grid.add_line(l);
            }
        }

        let overlaps = grid.overlaps();
        assert_eq!(5, overlaps.len());
        assert_eq!(true, overlaps.contains(&Point::new(3, 4)));
        assert_eq!(true, overlaps.contains(&Point::new(7, 4)));
        assert_eq!(true, overlaps.contains(&Point::new(0, 9)));
        assert_eq!(true, overlaps.contains(&Point::new(1, 9)));
        assert_eq!(true, overlaps.contains(&Point::new(2, 9)));
    }
}