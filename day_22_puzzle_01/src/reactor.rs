use crate::cube::{Cuboid, CuboidSet};
use regex::Regex;

pub struct Reactor {
    cubes: CuboidSet,
}

impl Reactor {
    pub fn new() -> Reactor {
        Reactor {
            cubes: vec![],
        }
    }

    pub fn turn_on(self, area: Cuboid) -> Reactor {
        let mut cubes = self.subtract(&area);
        cubes.push(area);
        Reactor {
            cubes
        }
    }

    pub fn turn_off(self, area: Cuboid) -> Reactor {
        let cubes = self.subtract(&area);
        Reactor {
            cubes
        }
    }

    fn subtract(self, area: &Cuboid) -> CuboidSet {
        let mut cubes = vec![];
        for c in self.cubes.into_iter() {
            cubes.extend(c.subtract(&area));
        }
        cubes
    }

    pub fn run_command(self, input: &str) -> Reactor {
        lazy_static! {
            static ref CUBOID_REGEX: Regex = Regex::new(r"x=(?P<start_x>-?\d+)\.\.(?P<end_x>-?\d+),\s*y=(?P<start_y>-?\d+)\.\.(?P<end_y>-?\d+),\s*z=(?P<start_z>-?\d+)\.\.(?P<end_z>-?\d+)").unwrap();
        }

        let captures = CUBOID_REGEX.captures(input);
        if let Some(captures) = captures {
            let parse_number = |n| i32::from_str_radix(n, 10).unwrap();

            let area = Cuboid::new(
                parse_number(&captures["start_x"])..=parse_number(&captures["end_x"]),
                parse_number(&captures["start_y"])..=parse_number(&captures["end_y"]),
                parse_number(&captures["start_z"])..=parse_number(&captures["end_z"])
            );
            if input.starts_with("on") {
                self.turn_on(area)
            } else if input.starts_with("off") {
                self.turn_off(area)
            } else {
                panic!("Unknown command - expected 'on' or 'off', found '{}'", input)
            }
        } else {
            panic!("Failed to parse on/off cuboid from '{}'", input)
        }
    }

    pub fn cube_count(&self) -> i64 {
        self.cubes.iter().map(|c| c.area()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tiny() {
        let r_step_0 = Reactor::new();
        let r_step_1 = r_step_0.turn_on(Cuboid::new(10..=12, 10..=12, 10..=12));
        let r_step_2 = r_step_1.turn_on(Cuboid::new(11..=13, 11..=13, 11..=13));
        let r_step_3 = r_step_2.turn_off(Cuboid::new(9..=11, 9..=11, 9..=11));
        let r_step_4 = r_step_3.turn_on(Cuboid::new(10..=10, 10..=10, 10..=10));

        assert_eq!(39, r_step_4.cube_count());
    }

    #[test]
    fn example_small_init() {
        let reactor = Reactor::new();

        let input = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15";

        let commands = input
            .split("\n")
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.trim())
            .collect::<Vec<_>>();
        
        let mut r: Option<Reactor> = Some(reactor);
        for command in commands.iter() {
            r = Some(r.unwrap().run_command(&command));
        }

        assert_eq!(590784, r.unwrap().cube_count());
    }

    #[test]
    fn example_small_reboot() {
        let reactor = Reactor::new();

        let input = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682";

        let commands = input
            .split("\n")
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.trim())
            .collect::<Vec<_>>();
        
        let mut r: Option<Reactor> = Some(reactor);
        for command in commands.iter() {
            r = Some(r.unwrap().run_command(&command));
        }

        assert_eq!(590784, r.unwrap().cube_count());
    }
}