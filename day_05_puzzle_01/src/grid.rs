use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
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

        // Failed to find 'Point { x: 213, y: 597 }' in wrong results
        // 213,740 -> 213,167
        // Failed to find intersections between:
        // line a: Line { start: Point { x: 213, y: 740 }, end: Point { x: 213, y: 167 } }
        // line b: Line { start: Point { x: 166, y: 335 }, end: Point { x: 561, y: 335 } }
        let target_line_a = Line::new(Point::new(213, 740), Point::new(213, 167));
        let target_line_b = Line::new(Point::new(166, 335), Point::new(561, 335));
        let mut compare_counter = 0;
        'outer: for outer in 0..(lines.len()-1) {
            'inner: for inner in (outer+1)..lines.len() {
                let a = lines[outer];
                let b = lines[inner];
                // if *a != target_line {
                //     continue 'outer;
                // } else if *b != target_line {
                //     continue 'inner;
                // }
                if *a != target_line_a {
                    continue 'outer;
                }
                if *b != target_line_b {
                    continue 'inner;
                }
                compare_counter += 1;
                let points = line::intersections_optimized(a, b);
                if points.len() > 0 {
                    println!("Found intersection between lines\nline a: {:?}\nline b: {:?}", a, b);
                    // for p in points.into_iter() {
                    //     overlaps.insert(p);
                    // }
                    // break 'outer;
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
    fn test_demo() {
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

    #[test]
    fn test_sample() {
        let file = File::open("./input.txt").unwrap();
        let lines = BufReader::new(file).lines();

        let mut grid = Grid::new();
        for line in lines {
            if let Ok(entry) = line {
                if entry.trim().len() == 0 {
                    // skip any rows with no content
                    continue;
                }
                let l = Line::from(entry.trim());
                if l.horizontal() || l.vertical() {
                    grid.add_line(l);
                }
            }
        }

        let overlaps = grid.overlaps();
        assert_eq!(4728, overlaps.len());
    }

    #[test]
    fn mismatch() {
        for correct_line in BufReader::new(File::open("./correct_result.txt").unwrap()).lines() {
            if let Ok(correct) = correct_line {
                let mut match_found = false;
                for wrong_line in BufReader::new(File::open("./wrong_result.txt").unwrap()).lines() {
                    if let Ok(wrong) = wrong_line {
                        if correct == wrong {
                            match_found = true;
                            break;
                        }
                    }
                }
                assert_eq!(true, match_found, "Failed to find '{}' in wrong results", correct);
            }
        }
    }
}