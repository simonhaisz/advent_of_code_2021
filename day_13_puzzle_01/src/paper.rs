type Dot = (usize, usize);

pub struct PaperBuilder {
    dots: Vec<Dot>,
}

impl PaperBuilder {
    pub fn new() -> PaperBuilder {
        PaperBuilder {
            dots: vec![],
        }
    }

    pub fn add_dot(&mut self, x: usize, y: usize) {
        self.dots.push((x, y));
    }

    pub fn build(self) -> Paper {
        Paper::new(self.dots)
    }
}

pub struct Paper {
    dots: Vec<Dot>,
}

impl Paper {
    pub fn new(dots: Vec<Dot>) -> Paper {
        Paper {
            dots,
        }
    }

    pub fn len(&self) -> usize {
        self.dots.len()
    }

    pub fn fold_horizontal(self, row: usize) -> Paper {
        self.fold(row, fold_dot_horizontal)
    }

    pub fn fold_vertical(self, column: usize) -> Paper {
        self.fold(column, fold_dot_vertical)
    }

    fn fold(self, offset: usize, fold_dot: fn(Dot, usize) -> Dot) -> Paper {
        let mut folded_dots = vec![];

        let mut add_dot = |d: Dot| {
            if !folded_dots.contains(&d) {
                folded_dots.push(d);
            }
        };

        for dot in self.dots.into_iter() {

            let folded_dot = fold_dot(dot, offset);

            add_dot(folded_dot);
        }

        Paper::new(folded_dots)
    }
}

fn fold_dot_horizontal(dot: Dot, row: usize) -> Dot {
    if dot.1 > row {
        let offset = (dot.1 - row) * 2;
        (dot.0, dot.1 - offset)
    } else if dot.1 < row {
        dot.clone()
    } else {
        panic!("Folding on a dot is not supported - row {} intersects with dot ({},{})", row, dot.0, dot.1)
    }
}

fn fold_dot_vertical(dot: Dot, column: usize) -> Dot {
    if dot.0 > column {
        let offset = (dot.0 - column) * 2;
        (dot.0 - offset, dot.1)
    } else if dot.0 < column {
        dot.clone()
    } else {
        panic!("Folding on a dot is not supported - column {} intersects with dot ({},{})", column, dot.0, dot.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_horizontal_demo_1() {
        let mut builder = PaperBuilder::new();
        builder.add_dot(6, 10);
        builder.add_dot(0, 14);
        builder.add_dot(9, 10);
        builder.add_dot(0, 3);
        builder.add_dot(10, 4);
        builder.add_dot(4, 11);
        builder.add_dot(6, 0);
        builder.add_dot(6, 12);
        builder.add_dot(4, 1);
        builder.add_dot(0, 13);
        builder.add_dot(10, 12);
        builder.add_dot(3, 4);
        builder.add_dot(3, 0);
        builder.add_dot(8, 4);
        builder.add_dot(1, 10);
        builder.add_dot(2, 14);
        builder.add_dot(8, 10);
        builder.add_dot(9, 0);

        let paper = builder.build();
        let folded = paper.fold_horizontal(7);

        assert_eq!(17, folded.dots.len());
    }

    #[test]
    fn test_flip_vertical_demo_1() {
        let mut builder = PaperBuilder::new();
        builder.add_dot(0, 0);
        builder.add_dot(2, 0);
        builder.add_dot(3, 0);
        builder.add_dot(6, 0);
        builder.add_dot(9, 0);
        builder.add_dot(0, 1);
        builder.add_dot(4, 1);
        builder.add_dot(6, 2);
        builder.add_dot(10, 2);
        builder.add_dot(0, 3);
        builder.add_dot(4, 3);
        builder.add_dot(1, 4);
        builder.add_dot(3, 4);
        builder.add_dot(6, 4);
        builder.add_dot(8, 4);
        builder.add_dot(9, 4);
        builder.add_dot(10, 4);

        let paper = builder.build();
        let folded = paper.fold_vertical(5);

        assert_eq!(16, folded.dots.len());
    }
    
}