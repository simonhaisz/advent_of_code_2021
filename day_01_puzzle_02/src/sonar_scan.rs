static SLIDING_WINDOW_SIZE: i32 = 3;

struct SlidingWindowState {
    pub depth_sum: i32,
    pub depth_count: i32,
}

impl SlidingWindowState {
    fn new() -> SlidingWindowState {
        SlidingWindowState {
            depth_sum: 0,
            depth_count: 0,
        }
    }

    fn add(&mut self, depth: i32) -> bool {
        if self.depth_count < SLIDING_WINDOW_SIZE {
            self.depth_sum += depth;
            self.depth_count += 1;
        }
        self.depth_count == SLIDING_WINDOW_SIZE
    }
}

pub struct SonarScan {
    sliding_windows: Vec<SlidingWindowState>,
    depth_increase_count: i32,
}

impl SonarScan {
    pub fn new() -> SonarScan {
        SonarScan {
            sliding_windows: vec!(),
            depth_increase_count: 0,
        }
    }

    pub fn depth_increase_count(&self) -> i32 {
        self.depth_increase_count
    }

    pub fn process_depth(&mut self, next_depth: i32) {
        let mut filled_windows = 0;
        for window in self.sliding_windows.iter_mut() {
            if window.add(next_depth) {
                filled_windows += 1;
            }
        }
        let mut new_window = SlidingWindowState::new();
        new_window.add(next_depth);
        self.sliding_windows.push(new_window);

        if filled_windows > 1 {
            let first = &self.sliding_windows[0];
            let second = &self.sliding_windows[1];
            if second.depth_sum > first.depth_sum {
                self.depth_increase_count += 1;
            }
            self.sliding_windows.remove(0);
        }
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
        scan.process_depth(100); // A
        scan.process_depth(110); // A B
        scan.process_depth(120); // A B C
        scan.process_depth(110); //   B C D
        scan.process_depth(100); //     C D E
        scan.process_depth(140); //       D E
        assert_eq!(2, scan.depth_increase_count());
    }
}