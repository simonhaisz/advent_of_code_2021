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

    pub fn cube_count(&self, apply_limit: bool) -> i64 {
        self.cubes.iter().map(|c| c.area(apply_limit)).sum()
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

        assert_eq!(39, r_step_4.cube_count(false));
    }

    #[test]
    fn example() {

        let input = "on x=-5..47,y=-31..22,z=-19..33
        on x=-44..5,y=-27..21,z=-14..35
        on x=-49..-1,y=-11..42,z=-10..38
        on x=-20..34,y=-40..6,z=-44..1
        off x=26..39,y=40..50,z=-2..11
        on x=-41..5,y=-41..6,z=-36..8
        off x=-43..-33,y=-45..-28,z=7..25
        on x=-33..15,y=-32..19,z=-34..11
        off x=35..47,y=-46..-34,z=-11..5
        on x=-14..36,y=-6..44,z=-16..29
        on x=-57795..-6158,y=29564..72030,z=20435..90618
        on x=36731..105352,y=-21140..28532,z=16094..90401
        on x=30999..107136,y=-53464..15513,z=8553..71215
        on x=13528..83982,y=-99403..-27377,z=-24141..23996
        on x=-72682..-12347,y=18159..111354,z=7391..80950
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
        on x=-52752..22273,y=-49450..9096,z=54442..119054
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471
        on x=-4958..62750,y=40422..118853,z=-7672..65583
        on x=55694..108686,y=-43367..46958,z=-26781..48729
        on x=-98497..-18186,y=-63569..3412,z=1232..88485
        on x=-726..56291,y=-62629..13224,z=18033..85226
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058
        on x=-65152..-11147,y=22489..91432,z=-58782..1780
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308
        on x=-57817..18248,y=49321..117703,z=5745..55881
        on x=14781..98692,y=-1341..70827,z=15753..70151
        on x=-34419..55919,y=-19626..40991,z=39015..114138
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915
        on x=-32178..58085,y=17647..101866,z=-91405..-8878
        on x=-53655..12091,y=50097..105568,z=-75335..-4862
        on x=-111166..-40997,y=-71714..2688,z=5609..50954
        on x=-16602..70118,y=-98693..-44401,z=5197..76897
        on x=16383..101554,y=4615..83635,z=-44907..18747
        off x=-95822..-15171,y=-19987..48940,z=10804..104439
        on x=-89813..-14614,y=16069..88491,z=-3297..45228
        on x=41075..99376,y=-20427..49978,z=-52012..13762
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197
        on x=-16478..35915,y=36008..118594,z=-7885..47086
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
        off x=2032..69770,y=-71013..4824,z=7471..94418
        on x=43670..120875,y=-42068..12382,z=-24787..38892
        off x=37514..111226,y=-45862..25743,z=-16714..54663
        off x=25699..97951,y=-30668..59918,z=-15349..69697
        off x=-44271..17935,y=-9516..60759,z=49131..112598
        on x=-61695..-5813,y=40978..94975,z=8655..80240
        off x=-101086..-9439,y=-7088..67543,z=33935..83858
        off x=18020..114017,y=-48931..32606,z=21474..89843
        off x=-77139..10506,y=-89994..-18797,z=-80..59318
        off x=8476..79288,y=-75520..11602,z=-96624..-24783
        on x=-47488..-1262,y=24338..100707,z=16292..72967
        off x=-84341..13987,y=2429..92914,z=-90671..-1318
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
        off x=-27365..46395,y=31009..98017,z=15428..76570
        off x=-70369..-16548,y=22648..78696,z=-1892..86821
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

        let commands = input
            .split("\n")
            .filter(|c| !c.trim().is_empty())
            .map(|c| c.trim())
            .collect::<Vec<_>>();
        
        let mut reactor = Reactor::new();
        for command in commands.iter() {
            reactor = reactor.run_command(&command);
        }

        assert_eq!(2758514936282235, reactor.cube_count(false));
        assert_eq!(474140, reactor.cube_count(true));
    }
}