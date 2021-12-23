use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::geometry::{Position, Rectangle};
use crate::triangle_number::TriangleNumber;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector {
    pos: Position,
    velocity_x: i32,
    velocity_y: i32,
}

impl Vector {
    pub fn new(pos: Position, velocity_x: i32, velocity_y: i32) -> Vector {
        Vector {
            pos,
            velocity_x,
            velocity_y,
        }
    }

    pub fn next_step(&self) -> Vector {
        let next_pos = Position::new(self.pos.x() + self.velocity_x, self.pos.y() + self.velocity_y);
        let next_velocity_x = if self.velocity_x > 0 { self.velocity_x - 1 } else if self.velocity_x == 0 { 0 } else { panic!("x velocity should never be zero") };
        let next_velocity_y = self.velocity_y - 1;
        Vector::new(next_pos, next_velocity_x, next_velocity_y)
    }
}

fn find_valid_horizontal_velocities(distance_range: (i32, i32)) -> Vec<(i32, i32, i32)> {
    if distance_range.0 >= distance_range.1 {
        panic!("Distance range should go from closest to furthest and they should not equal - found ({}, {})", distance_range.0, distance_range.1)
    }
    let max_velocity = distance_range.1;

    let mut hits = vec![];

    'velocities: for initial_velocity in 1..=max_velocity {
        let mut d = 0;
        let mut v = initial_velocity;
        for step in 1.. {
            d += v;
            v -= 1;
            if d >= distance_range.0 && d <= distance_range.1 {
                hits.push((step, initial_velocity, v));
            }
            if d > distance_range.1 || v == 0 {
                continue 'velocities;
            }
        }
    }

    hits
}

fn find_hit_vertical_velocities(distance_range: (i32, i32), steps: i32) -> Vec<i32> {
    if distance_range.0 <= distance_range.1 {
        panic!("Distance range should go from closest to furthest and they should not equal - found ({}, {})", distance_range.0, distance_range.1)
    }

    if steps == 0 {
        panic!("Zero steps means we are already in the target area - danger close!")
    }

    let mut velocities = vec![];

    for velocity in 0.. {
        let gravity = 1 - steps;
        let fall = velocity * steps + gravity.triangle_number();
        if fall >= distance_range.1 && fall <= distance_range.0 {
            velocities.push(velocity);
        } else if fall > distance_range.0 {
            break;
        }
    }

    let mut velocity = -1;
    loop {
        let gravity = 1 - steps;
        let fall = velocity * steps + gravity.triangle_number();
        if fall >= distance_range.1 && fall <= distance_range.0 {
            velocities.push(velocity);
        } else if fall < distance_range.1 {
            break;
        }
        velocity -= 1;
    }

    velocities
}

pub fn hit_arc(launch: &Vector, target: &Rectangle) -> Option<Vec<Vector>> {
    let mut ballistic_arc = vec![];

    if target.within(&launch.pos) {
        panic!("Danger close! Launching within the target zone")
    }

    let mut current = launch.clone();

    loop {
        ballistic_arc.push(current.clone());

        if target.within(&current.pos) {
            return Some(ballistic_arc);
        } else if target.far(&current.pos) {
            return None;
        } else {
            current = current.next_step();
        }
    }
}

pub fn find_all_hit_launches(target: &Rectangle) -> HashSet<Vector> {

    let horizontal_velocities = find_valid_horizontal_velocities(target.horizontal_range());

    let mut exact_steps = HashSet::new();
    let mut lowest_min_step = i32::MAX;
    for hori_vel in horizontal_velocities.iter() {
        if hori_vel.2 > 0 {
            exact_steps.insert(hori_vel.0);
        } else {
            lowest_min_step = cmp::min(lowest_min_step, hori_vel.0);
        }
    }

    let mut step_vertical_velocities = HashMap::new();

    for exact in exact_steps.iter() {
        let vertical_velocities = find_hit_vertical_velocities(target.vertical_range(), *exact);
        if vertical_velocities.len() > 0 {
            step_vertical_velocities.insert(*exact, vertical_velocities);
        }
    }

    // FIXME: how to determine max steps?
    let max_steps = 1000;
    for step in lowest_min_step..max_steps {
        if step_vertical_velocities.get(&step).is_none() {
            let vertical_velocities = find_hit_vertical_velocities(target.vertical_range(), step);
            if vertical_velocities.len() > 0 {
                step_vertical_velocities.insert(step, vertical_velocities);
            }
        }
    }

    let mut hit_launches = HashSet::new();

    let mut find_hits = |&steps, horizontal| {
        if let Some(verticals) = step_vertical_velocities.get(&steps) {
            for vertical in verticals.iter() {
                let launch = Vector::new(Position::origin(), horizontal, *vertical);
                let hit = hit_arc(&launch, &target);
                if hit.is_some() {
                    hit_launches.insert(launch);
                }
            }
        }
    };

    for (steps, horizontal, horizontal_hit) in horizontal_velocities.iter() {
        find_hits(steps, *horizontal);
        if *horizontal_hit == 0 {
            for valid_steps in step_vertical_velocities.keys() {
                if *valid_steps > *steps {
                    find_hits(valid_steps, *horizontal);
                }
            }
        }
    }

    hit_launches
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector_next() {
        let origin = Vector::new(Position::origin(), 5, 5);

        let step_1 = origin.next_step();
        assert_eq!(Vector::new(Position::new(5, 5), 4, 4), step_1);

        let step_2 = step_1.next_step();
        assert_eq!(Vector::new(Position::new(9, 9), 3, 3), step_2);

        let step_3 = step_2.next_step();
        assert_eq!(Vector::new(Position::new(12, 12), 2, 2), step_3);

        let step_4 = step_3.next_step();
        assert_eq!(Vector::new(Position::new(14, 14), 1, 1), step_4);

        let step_5 = step_4.next_step();
        assert_eq!(Vector::new(Position::new(15, 15), 0, 0), step_5);

        let step_6 = step_5.next_step();
        assert_eq!(Vector::new(Position::new(15, 15), 0, -1), step_6);

        let step_7 = step_6.next_step();
        assert_eq!(Vector::new(Position::new(15, 14), 0, -2), step_7);

        let step_8 = step_7.next_step();
        assert_eq!(Vector::new(Position::new(15, 12), 0, -3), step_8);

        let step_9 = step_8.next_step();
        assert_eq!(Vector::new(Position::new(15, 9), 0, -4), step_9);

        let step_10 = step_9.next_step();
        assert_eq!(Vector::new(Position::new(15, 5), 0, -5), step_10);
    }

    #[test]
    fn test_example_1() {
        let launch = Vector::new(Position::origin(), 7, 2);

        let target = Rectangle::new(20, 30, -10, -5);

        let arc = hit_arc(&launch, &target);

        assert_eq!(true, arc.is_some());
        assert_eq!(Position::new(28, -7), arc.unwrap().last().unwrap().pos);
    }

    #[test]
    fn test_example_2() {
        let launch = Vector::new(Position::origin(), 6, 3);

        let target = Rectangle::new(20, 30, -10, -5);

        let arc = hit_arc(&launch, &target);

        assert_eq!(true, arc.is_some());
        assert_eq!(Position::new(21, -9), arc.unwrap().last().unwrap().pos);
    }

    #[test]
    fn test_example_3() {
        let launch = Vector::new(Position::origin(), 9, 0);

        let target = Rectangle::new(20, 30, -10, -5);

        let arc = hit_arc(&launch, &target);

        assert_eq!(true, arc.is_some());
        assert_eq!(Position::new(30, -6), arc.unwrap().last().unwrap().pos);
    }

    #[test]
    fn test_example_4() {
        let launch = Vector::new(Position::origin(), 17, -4);

        let target = Rectangle::new(20, 30, -10, -5);

        let arc = hit_arc(&launch, &target);

        assert_eq!(false, arc.is_some());
    }

    #[test]
    fn test_example_all_valid_launches() {
        let target = Rectangle::new(20, 30, -10, -5);

        let all_hit_launches = find_all_hit_launches(&target);

        let input = "
        23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
        25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
        8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
        26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
        20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
        25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
        25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
        8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
        24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
        7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
        23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
        27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
        8,-2    27,-8   30,-5   24,-7"
            .replace("\n", "");

        let pairs: Vec<&str> = input
            .split(" ")
            .filter(|v| !v.is_empty())
            .collect();
        
        let mut actual_hit_launches = HashSet::new();
        for pair in pairs.iter() {
            let v_pair: Vec<i32> = pair.split(",").map(|v| i32::from_str_radix(v, 10).unwrap()).collect();
            actual_hit_launches.insert(Vector::new(Position::origin(), v_pair[0], v_pair[1]));
        }

        assert_eq!(112, actual_hit_launches.len());

        let mut missing_launches = HashSet::new();
        for actual in actual_hit_launches.iter() {
            if all_hit_launches.get(&actual).is_none() {
                missing_launches.insert(actual.clone());
            }
        }
        assert_eq!(HashSet::new(), missing_launches);
    }

    #[test]
    fn tests_7_6() {
        let target = Rectangle::new(20, 30, -10, -5);
        let launch = Vector::new(Position::origin(), 6, 5);
        let hit = hit_arc(&launch, &target);
        assert_eq!(true, hit.is_some());

        let horizontal_velocities = find_valid_horizontal_velocities(target.horizontal_range());

        let seven_velocities = horizontal_velocities
            .iter()
            .filter(|(_, i_v, _)| *i_v == 7)
            .collect::<Vec<_>>();
        
        assert_eq!(4, seven_velocities.len());
    }
}