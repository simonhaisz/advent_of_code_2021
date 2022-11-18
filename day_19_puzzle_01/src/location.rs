use std::collections::HashMap;
use std::cmp;

const AXES_COUNT: usize = 3;

const OTHER_AXES: [[usize; 2]; 3] = [
    [1, 2],
    [2, 0],
    [0, 1]
];

const MIN_MATCHING_LOCATION_COUNT: usize = 12;

pub type Location = [i32; AXES_COUNT];

pub fn location_from(input: &str) -> Location {
    if input.split(",").count() != 3 {
        panic!("Locations should be triples - found {} values in '{}'", input.split(",").count(), input)
    }
    let parse = |v| i32::from_str_radix(v, 10).unwrap();
    let mut split = input.split(",");

    [
        parse(split.next().unwrap()),
        parse(split.next().unwrap()),
        parse(split.next().unwrap()),
    ]
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Angle {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy
}
pub type Rotation = [Angle; 3];

pub const UNIQUE_ROTATIONS: [Rotation; 24] = [
    [Angle::Zero, Angle::Zero, Angle::Zero],
    [Angle::Zero, Angle::Zero, Angle::Ninety],
    [Angle::Zero, Angle::Zero, Angle::OneEighty],
    [Angle::Zero, Angle::Zero, Angle::TwoSeventy],
    [Angle::Zero, Angle::Ninety, Angle::Zero],
    [Angle::Zero, Angle::Ninety, Angle::Ninety],
    [Angle::Zero, Angle::Ninety, Angle::OneEighty],
    [Angle::Zero, Angle::Ninety, Angle::TwoSeventy],
    [Angle::Zero, Angle::OneEighty, Angle::Zero],
    [Angle::Zero, Angle::OneEighty, Angle::Ninety],
    [Angle::Zero, Angle::OneEighty, Angle::OneEighty],
    [Angle::Zero, Angle::OneEighty, Angle::TwoSeventy],
    [Angle::Zero, Angle::TwoSeventy, Angle::Zero],
    [Angle::Zero, Angle::TwoSeventy, Angle::Ninety],
    [Angle::Zero, Angle::TwoSeventy, Angle::OneEighty],
    [Angle::Zero, Angle::TwoSeventy, Angle::TwoSeventy],
    [Angle::Ninety, Angle::Zero, Angle::Zero],
    [Angle::Ninety, Angle::Zero, Angle::Ninety],
    [Angle::Ninety, Angle::Zero, Angle::OneEighty],
    [Angle::Ninety, Angle::Zero, Angle::TwoSeventy],
    [Angle::Ninety, Angle::OneEighty, Angle::Zero],
    [Angle::Ninety, Angle::OneEighty, Angle::Ninety],
    [Angle::Ninety, Angle::OneEighty, Angle::OneEighty],
    [Angle::Ninety, Angle::OneEighty, Angle::TwoSeventy],
];

pub fn relative_to(location: &Location, origin: &Location) -> Location {
    [
        location[0] - origin[0],
        location[1] - origin[1],
        location[2] - origin[2]
    ]
}

pub fn distance_between(a: &Location, b: &Location) -> i32 {
    let mut distance = 0;

    for i in 0..3 {
        distance += (a[i] - b[i]).abs();
    }

    distance
}

pub fn rotate_by(location: &Location, rotation: &Rotation) -> Location {
    let mut rotated = location.clone();
    for (axis, angle) in rotation.iter().enumerate() {
        rotate_on_axis(&mut rotated, axis, *angle);
    }
    rotated
}

fn rotate_on_axis(location: &mut Location, axis: usize, angle: Angle) {
    if axis > 2 {
        panic!("There are only three axes - expected 0, 1, or 2, found {}", axis)
    }
    if angle != Angle::Zero {
        let reference = location.clone();
        let other_axes = OTHER_AXES[axis];
        match angle {
            Angle::Ninety => {
                location[other_axes[0]] = reference[other_axes[1]];
                location[other_axes[1]] = -reference[other_axes[0]];
            },
            Angle::OneEighty => {
                location[other_axes[0]] = -reference[other_axes[0]];
                location[other_axes[1]] = -reference[other_axes[1]];
            },
            Angle::TwoSeventy => {
                location[other_axes[0]] = -reference[other_axes[1]];
                location[other_axes[1]] = reference[other_axes[0]];
            },
            _ => panic!("Should only be rotating 90, 180, or 270 degrees - found {:#?}", angle),
        }
    }
}

fn locations_from(input: &str) -> Vec<Location> {
    input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l| location_from(l.trim()))
        .collect()
}

fn sort_locations(locations: &mut Vec<Location>) {
    locations.sort_by(|a, b| {
        let a_sum: i32 = a.iter().map(|v| v.abs()).sum();
        let b_sum: i32 = b.iter().map(|v| v.abs()).sum();

        if a_sum == b_sum {
            let mut cmp = std::cmp::Ordering::Equal;

            for i in 0..3 {
                let axis_cmp = a[i].abs().cmp(&b[i].abs());
                if axis_cmp != std::cmp::Ordering::Equal {
                    cmp = axis_cmp;
                    break;
                }
            }
            cmp
        } else {
            a_sum.cmp(&b_sum)
        }
    });
}

fn simplify_locations(locations: &Vec<Location>) -> Vec<Location> {
    // let locations = locations.clone();

    // sort_locations(&mut locations);

    let relative_origin = locations[0];

    let relative = locations
        .iter()
        .map(|l| relative_to(&l, &relative_origin))
        .collect();

    relative
}

fn locations_match(locations_a: &Vec<Location>, locations_b: &Vec<Location>) -> bool {
    let mut matching_locations = 0;
    
    'a: for a in locations_a.iter() {
        for b in locations_b.iter() {
            if distance_between(a, b) == 0 {
                matching_locations += 1;
                continue 'a;
            }
        }
    }

    if matching_locations > 0 {
        matching_locations >= MIN_MATCHING_LOCATION_COUNT
    } else {
        false
    }
}

fn sets_match(locations_a: &Vec<Location>, locations_b: &Vec<Location>) -> bool {
    let matrix_a = distance_matrix(&locations_a);
    let matrix_b = distance_matrix(&locations_b);

    if distance_matrices_match(&matrix_a, &matrix_b) {
        for r in UNIQUE_ROTATIONS.iter() {
            let roated_b = locations_b
                .iter()
                .map(|l| rotate_by(&l, &r))
                .collect();
    
            if locations_match(&locations_a, &roated_b) {
                return true;
            }
        }
    }

    false
}

fn unique_rotations(location: &Location) -> HashMap<Location, Vec<Rotation>> {
    let all_angles = [Angle::Zero, Angle::Ninety, Angle::OneEighty, Angle::TwoSeventy];

    let mut unique_rotations: HashMap<Location, Vec<Rotation>> = HashMap::new();

    let mut count = 0;
    for x in all_angles.iter() {
        for y in all_angles.iter() {
            for z in all_angles.iter() {
                count += 1;
                let rotation = [*x, *y, *z];
                let rotated = rotate_by(&location, &rotation);

                println!("{}: {:?}", count, rotated);

                unique_rotations.entry(rotated).or_insert(Vec::new()).push(rotation);
            }
        }
    }

    unique_rotations
}

pub type LocationPair = (Location, Location);
pub type DistanceMatrix = HashMap<i32, Vec<LocationPair>>;

pub fn distance_matrix(locations: &Vec<Location>) -> DistanceMatrix {

    let mut matrix = HashMap::new();

    let locations_count = locations.len();

    for outer_index in 0..locations_count - 1 {
        for inner_index in outer_index + 1..locations_count {
            let outer = locations[outer_index];
            let inner = locations[inner_index];
            let distance = distance_between(&outer, &inner);

            let entry = matrix.entry(distance).or_insert(Vec::new());
            entry.push((outer, inner));
        }
    }

    matrix
}

pub fn distance_matrices_match(matrix_a: &DistanceMatrix, matrix_b: &DistanceMatrix) -> bool {
    let mut matching_distance_count = 0;

    for (distance, pairs) in matrix_a.iter() {
        if let Some(other_pairs) = matrix_b.get(distance) {
            matching_distance_count += cmp::min(pairs.len(), other_pairs.len());
        }
    }

    matching_distance_count >= MIN_MATCHING_LOCATION_COUNT
}

pub fn distance_matrices_common_pairs(matrix_a: &DistanceMatrix, matrix_b: &DistanceMatrix) -> HashMap<i32, Vec<(LocationPair, LocationPair)>> {
    let mut common

    for (distance, pairs) in matrix_a.iter() {
        if let Some(other_pairs) = matrix_b.get(distance) {
            matching_distance_count += cmp::min(pairs.len(), other_pairs.len());
        }
    }

    matching_distance_count >= MIN_MATCHING_LOCATION_COUNT
}

#[cfg(test)]
mod tests {
    use super::*;
    use Angle::*;

    #[test]
    fn relative_to_zero() {
        let location = [100, 100, 100];
        let origin = [0, 0, 0];
        assert_eq!([100, 100, 100], relative_to(&location, &origin));
    }

    #[test]
    fn relative_to_after() {
        let location = [100, 100, 100];
        let origin = [10, 20, 30];
        assert_eq!([90, 80, 70], relative_to(&location, &origin));
    }

    #[test]
    fn relative_to_before() {
        let location = [-100, -100, -100];
        let origin = [50, -10, 20];
        assert_eq!([-150, -90, -120], relative_to(&location, &origin));
    }

    #[test]
    fn rotate_on_axis_x() {
        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 0, Angle::Zero);
        assert_eq!([10, 20, 30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 0, Angle::Ninety);
        assert_eq!([10, 30, -20], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 0, Angle::OneEighty);
        assert_eq!([10, -20, -30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 0, Angle::TwoSeventy);
        assert_eq!([10, -30, 20], location);
    }

    #[test]
    fn rotate_on_axis_y() {
        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 1, Angle::Zero);
        assert_eq!([10, 20, 30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 1, Angle::Ninety);
        assert_eq!([-30, 20, 10], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 1, Angle::OneEighty);
        assert_eq!([-10, 20, -30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 1, Angle::TwoSeventy);
        assert_eq!([30, 20, -10], location);
    }

    #[test]
    fn rotate_on_axis_z() {
        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 2, Angle::Zero);
        assert_eq!([10, 20, 30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 2, Angle::Ninety);
        assert_eq!([20, -10, 30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 2, Angle::OneEighty);
        assert_eq!([-10, -20, 30], location);

        let mut location = [10, 20, 30];
        rotate_on_axis(&mut location, 2, Angle::TwoSeventy);
        assert_eq!([-20, 10, 30], location);
    }

    #[test]
    fn rotate_by_many() {
        let location = [10, 20, 30];

        // [30, -20, 10] from [[Zero, Ninety, OneEighty], [Ninety, Ninety, Ninety], [OneEighty, Ninety, Zero], [TwoSeventy, Ninety, TwoSeventy]]
        for r in [[Zero, Ninety, OneEighty], [Ninety, Ninety, Ninety], [OneEighty, Ninety, Zero], [TwoSeventy, Ninety, TwoSeventy]].into_iter() {
            assert_eq!([30, -20, 10], rotate_by(&location, &r));
        }

        // [10, 20, 30] from [[Zero, Zero, Zero], [OneEighty, OneEighty, OneEighty]]
        for r in [[Zero, Zero, Zero], [OneEighty, OneEighty, OneEighty]].into_iter() {
            assert_eq!([10, 20, 30], rotate_by(&location, &r));
        }

        // [-20, -30, 10] from [[Zero, Ninety, TwoSeventy], [Ninety, Ninety, OneEighty], [OneEighty, Ninety, Ninety], [TwoSeventy, Ninety, Zero]]
        for r in [[Zero, Ninety, TwoSeventy], [Ninety, Ninety, OneEighty], [OneEighty, Ninety, Ninety], [TwoSeventy, Ninety, Zero]].into_iter() {
            assert_eq!([-20, -30, 10], rotate_by(&location, &r));
        }

        // [-20, 30, -10] from [[Zero, TwoSeventy, TwoSeventy], [Ninety, TwoSeventy, Zero], [OneEighty, TwoSeventy, Ninety], [TwoSeventy, TwoSeventy, OneEighty]]
        for r in [[Zero, TwoSeventy, TwoSeventy], [Ninety, TwoSeventy, Zero], [OneEighty, TwoSeventy, Ninety], [TwoSeventy, TwoSeventy, OneEighty]].into_iter() {
            assert_eq!([-20, 30, -10], rotate_by(&location, &r));
        }

        // [10, 30, -20] from [[Ninety, Zero, Zero], [TwoSeventy, OneEighty, OneEighty]]
        for r in [[Ninety, Zero, Zero], [TwoSeventy, OneEighty, OneEighty]].into_iter() {
            assert_eq!([10, 30, -20], rotate_by(&location, &r));
        }

        // [30, -10, -20] from [[Ninety, Zero, Ninety], [TwoSeventy, OneEighty, TwoSeventy]]
        for r in [[Ninety, Zero, Ninety], [TwoSeventy, OneEighty, TwoSeventy]].into_iter() {
            assert_eq!([30, -10, -20], rotate_by(&location, &r));
        }

        // [-10, -30, -20] from [[Ninety, Zero, OneEighty], [TwoSeventy, OneEighty, Zero]]
        for r in [[Ninety, Zero, OneEighty], [TwoSeventy, OneEighty, Zero]].into_iter() {
            assert_eq!([-10, -30, -20], rotate_by(&location, &r));
        }

        // [-10, 30, 20] from [[Ninety, OneEighty, Zero], [TwoSeventy, Zero, OneEighty]]
        for r in [[Ninety, OneEighty, Zero], [TwoSeventy, Zero, OneEighty]].into_iter() {
            assert_eq!([-10, 30, 20], rotate_by(&location, &r));
        }

        // [10, -30, 20] from [[Ninety, OneEighty, OneEighty], [TwoSeventy, Zero, Zero]]
        for r in [[Ninety, OneEighty, OneEighty], [TwoSeventy, Zero, Zero]].into_iter() {
            assert_eq!([10, -30, 20], rotate_by(&location, &r));
        }

        // [20, 30, 10] from [[Zero, Ninety, Ninety], [Ninety, Ninety, Zero], [OneEighty, Ninety, TwoSeventy], [TwoSeventy, Ninety, OneEighty]]
        for r in [[Zero, Ninety, Ninety], [Ninety, Ninety, Zero], [OneEighty, Ninety, TwoSeventy], [TwoSeventy, Ninety, OneEighty]].into_iter() {
            assert_eq!([20, 30, 10], rotate_by(&location, &r));
        }

        // [-30, 20, 10] from [[Zero, Ninety, Zero], [Ninety, Ninety, TwoSeventy], [OneEighty, Ninety, OneEighty], [TwoSeventy, Ninety, Ninety]]
        for r in [[Zero, Ninety, Zero], [Ninety, Ninety, TwoSeventy], [OneEighty, Ninety, OneEighty], [TwoSeventy, Ninety, Ninety]].into_iter() {
            assert_eq!([-30, 20, 10], rotate_by(&location, &r));
        }

        // [30, 10, 20] from [[Ninety, OneEighty, Ninety], [TwoSeventy, Zero, TwoSeventy]]
        for r in [[Ninety, OneEighty, Ninety], [TwoSeventy, Zero, TwoSeventy]].into_iter() {
            assert_eq!([30, 10, 20], rotate_by(&location, &r));
        }

        // [-20, -10, -30] from [[Zero, OneEighty, TwoSeventy], [OneEighty, Zero, Ninety]]
        for r in [[Zero, OneEighty, TwoSeventy], [OneEighty, Zero, Ninety]].into_iter() {
            assert_eq!([-20, -10, -30], rotate_by(&location, &r));
        }

        // [-30, -10, 20] from [[Ninety, OneEighty, TwoSeventy], [TwoSeventy, Zero, Ninety]]
        for r in [[Ninety, OneEighty, TwoSeventy], [TwoSeventy, Zero, Ninety]].into_iter() {
            assert_eq!([-30, -10, 20], rotate_by(&location, &r));
        }

        // [-30, 10, -20] from [[Ninety, Zero, TwoSeventy], [TwoSeventy, OneEighty, Ninety]]
        for r in [[Ninety, Zero, TwoSeventy], [TwoSeventy, OneEighty, Ninety]].into_iter() {
            assert_eq!([-30, 10, -20], rotate_by(&location, &r));
        }

        // [30, 20, -10] from [[Zero, TwoSeventy, Zero], [Ninety, TwoSeventy, Ninety], [OneEighty, TwoSeventy, OneEighty], [TwoSeventy, TwoSeventy, TwoSeventy]]
        for r in [[Zero, TwoSeventy, Zero], [Ninety, TwoSeventy, Ninety], [OneEighty, TwoSeventy, OneEighty], [TwoSeventy, TwoSeventy, TwoSeventy]].into_iter() {
            assert_eq!([30, 20, -10], rotate_by(&location, &r));
        }

        // [-10, 20, -30] from [[Zero, OneEighty, Zero], [OneEighty, Zero, OneEighty]]
        for r in [[Zero, OneEighty, Zero], [OneEighty, Zero, OneEighty]].into_iter() {
            assert_eq!([-10, 20, -30], rotate_by(&location, &r));
        }

        // [-20, 10, 30] from [[Zero, Zero, TwoSeventy], [OneEighty, OneEighty, Ninety]]
        for r in [[Zero, Zero, TwoSeventy], [OneEighty, OneEighty, Ninety]].into_iter() {
            assert_eq!([-20, 10, 30], rotate_by(&location, &r));
        }

        // [20, -10, 30] from [[Zero, Zero, Ninety], [OneEighty, OneEighty, TwoSeventy]]
        for r in [[Zero, Zero, Ninety], [OneEighty, OneEighty, TwoSeventy]].into_iter() {
            assert_eq!([20, -10, 30], rotate_by(&location, &r));
        }

        // [20, -30, -10] from [[Zero, TwoSeventy, Ninety], [Ninety, TwoSeventy, OneEighty], [OneEighty, TwoSeventy, TwoSeventy], [TwoSeventy, TwoSeventy, Zero]]
        for r in [[Zero, TwoSeventy, Ninety], [Ninety, TwoSeventy, OneEighty], [OneEighty, TwoSeventy, TwoSeventy], [TwoSeventy, TwoSeventy, Zero]].into_iter() {
            assert_eq!([20, -30, -10], rotate_by(&location, &r));
        }

        // [-30, -20, -10] from [[Zero, TwoSeventy, OneEighty], [Ninety, TwoSeventy, TwoSeventy], [OneEighty, TwoSeventy, Zero], [TwoSeventy, TwoSeventy, Ninety]]
        for r in [[Zero, TwoSeventy, OneEighty], [Ninety, TwoSeventy, TwoSeventy], [OneEighty, TwoSeventy, Zero], [TwoSeventy, TwoSeventy, Ninety]].into_iter() {
            assert_eq!([-30, -20, -10], rotate_by(&location, &r));
        }

        // [-10, -20, 30] from [[Zero, Zero, OneEighty], [OneEighty, OneEighty, Zero]]
        for r in [[Zero, Zero, OneEighty], [OneEighty, OneEighty, Zero]].into_iter() {
            assert_eq!([-10, -20, 30], rotate_by(&location, &r));
        }

        // [20, 10, -30] from [[Zero, OneEighty, Ninety], [OneEighty, Zero, TwoSeventy]]
        for r in [[Zero, OneEighty, Ninety], [OneEighty, Zero, TwoSeventy]].into_iter() {
            assert_eq!([20, 10, -30], rotate_by(&location, &r));
        }

        // [10, -20, -30] from [[Zero, OneEighty, OneEighty], [OneEighty, Zero, Zero]]
        for r in [[Zero, OneEighty, OneEighty], [OneEighty, Zero, Zero]].into_iter() {
            assert_eq!([10, -20, -30], rotate_by(&location, &r));
        }
    }

    #[test]
    fn matrices_simple() {
        let a = locations_from("404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401");

        let b = locations_from("686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390");

        let matrix_a = distance_matrix(&a);
        let matrix_b = distance_matrix(&b);

        assert_eq!(true, distance_matrices_match(&matrix_a, &matrix_b));
    }

    #[test]
    fn sets_simple() {
        let a = locations_from("404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401");

        let b = locations_from("686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390");

        assert_eq!(true, sets_match(&a, &b));
    }
}