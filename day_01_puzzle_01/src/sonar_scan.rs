pub struct SonarScan {
    previous_depth: Option<i32>,
    depth_increase_count: i32,
}

impl SonarScan {
    pub fn new() -> SonarScan {
        SonarScan {
            previous_depth: None,
            depth_increase_count: 0,
        }
    }

    pub fn depth_increase_count(&self) -> i32 {
        self.depth_increase_count
    }

    pub fn process_depth(&mut self, next_depth: i32) {
        if let Some(depth) = self.previous_depth {
            if next_depth > depth {
                self.depth_increase_count += 1;
            }
        }

        self.previous_depth = Some(next_depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let scan = SonarScan::new();
        assert_eq!(0, scan.depth_increase_count());
    }

    #[test]
    fn test_skip_first_depth() {
        let mut scan = SonarScan::new();
        scan.process_depth(100);
        assert_eq!(0, scan.depth_increase_count());
    }

    #[test]
    fn test_multiple_depths() {
        let mut scan = SonarScan::new();
        scan.process_depth(100);
        scan.process_depth(110);
        scan.process_depth(120);
        scan.process_depth(90);
        scan.process_depth(90);
        scan.process_depth(99);
        assert_eq!(3, scan.depth_increase_count());
    }
}