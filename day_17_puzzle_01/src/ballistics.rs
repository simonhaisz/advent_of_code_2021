use crate::geometry::{Position, Rectangle};
use crate::triangle_number::TriangleNumber;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn p_y(&self) -> i32 {
        self.pos.y()
    }
}

fn find_max_steps_initial_velocity_x(distance_range: (i32, i32)) -> (i32, i32, i32) {
    if distance_range.0 >= distance_range.1 {
        panic!("Distance range should go from closest to furthest and they should not equal - found ({}, {})", distance_range.0, distance_range.1)
    }
    let max_velocity = distance_range.1;

    let mut hit: Option<(i32, i32, i32)> = None;

    'velocities: for initial_velocity in 1..max_velocity {
        let mut d = 0;
        let mut v = initial_velocity;
        for step in 1.. {
            d += v;
            v -= 1;
            if d >= distance_range.0 && d <= distance_range.1 {
                hit = Some((step, initial_velocity, v));
            }
            if d > distance_range.1 || v == 0 {
                if hit.is_some() {
                    break 'velocities;
                } else {
                    continue 'velocities;
                }
            }
        }
    }

    hit.unwrap()
}

fn find_max_initial_velocity_y(distance_range: (i32, i32), max_steps: Option<i32>) -> i32 {
    if distance_range.0 <= distance_range.1 {
        panic!("Distance range should go from closest to furthest and they should not equal - found ({}, {})", distance_range.0, distance_range.1)
    }

    if let Some(max_steps) = max_steps {
        if max_steps == 0 {
            panic!("Zero steps means we are already in the target area - danger close!")
        }

        let mut max_hit_velocity = 0;

        for velocity in 0.. {
            let gravity = max_steps - velocity;
            let fall = velocity.triangle_number() - gravity.triangle_number();
            if fall >= distance_range.0 && fall <= distance_range.1 {
                max_hit_velocity = velocity;
            } else if fall > distance_range.1 {
                break;
            }
        }

        max_hit_velocity

    } else {
        // if the projectile can be within the target area as its x velocity drops to zero then there is no step limit
        // just a limit on the y velocity becoming so negative that it passes through the target area without 'stopping' in it
        // and since the origin is 0,0 and the parabolic arc is perfect (in integer-land anyways) it is guarenteed to hit 0 again
        // where it's speed will be one higher than it's initial speed
        distance_range.1.abs() - 1
    }
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

pub fn find_fanciest_hit_arc(target: &Rectangle) -> Option<Vec<Vector>> {

    let (steps, initial_velocity_x, resulting_velocity_x) = find_max_steps_initial_velocity_x(target.horizontal_range());

    let max_steps = if resulting_velocity_x > 0 {
        Some(steps)
    } else {
        None
    };

    let initial_velocity_y = find_max_initial_velocity_y(target.vertical_range(), max_steps);

    let launch = Vector::new(Position::origin(), initial_velocity_x, initial_velocity_y);

    hit_arc(&launch, target)
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
    fn test_example_fanciest() {
        let target = Rectangle::new(20, 30, -10, -5);

        let fanciest_arc = find_fanciest_hit_arc(&target).unwrap();
        assert_eq!(6, fanciest_arc[0].velocity_x);
        assert_eq!(9, fanciest_arc[0].velocity_y);
    }
}