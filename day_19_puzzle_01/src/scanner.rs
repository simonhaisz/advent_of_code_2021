use crate::location::Location;

pub struct Scanner {
    beacons: Vec<Location>,
}

impl Scanner {
    pub fn new(beacons: Vec<Location>) -> Scanner {
        Scanner {
            beacons
        }
    }
}