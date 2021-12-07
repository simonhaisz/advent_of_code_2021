use std::collections::HashMap;

pub struct SchoolOfFish {
    all_generations: HashMap<u32,HashMap<u32, u64>>,
}

impl SchoolOfFish {
    pub fn from(input: &str) -> SchoolOfFish {
        let mut initial_generation = HashMap::new();
        for n in parse_input(input).iter() {
            if let Some(count) = initial_generation.get_mut(n) {
                *count += 1;
            } else {
                initial_generation.insert(*n, 1);
            }
        }
        let mut all_generations = HashMap::new();
        all_generations.insert(6, initial_generation);
        SchoolOfFish {
            all_generations
        }
    }

    pub fn len(&self) -> u64 {
        let mut total_count = 0;
        for (_, generation) in &self.all_generations {
            for (_, count) in generation {
                total_count += count;
            }
        }
        total_count
    }

    pub fn next_day(&mut self) {
        let mut next_all_generations = HashMap::new();

        for (timer_delay, current_generation) in self.all_generations.iter() {
            for (timer, count) in current_generation.iter() {
                {
                    let next_generation = next_all_generations.entry(*timer_delay).or_insert(HashMap::new());
                    let c = if *timer > 0 {
                        next_generation.entry(*timer - 1).or_insert(0)
                    } else {
                        next_generation.entry(6).or_insert(0)
                    };
                    *c += *count;
                }
                {
                    if *timer == 0 {
                        let child_generation = next_all_generations.entry(8).or_insert(HashMap::new());
                        let c = child_generation.entry(8).or_insert(0);
                        *c += *count;
                    }
                }
            }
        }

        self.all_generations = next_all_generations;
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(",")
        .filter(|v| !v.is_empty())
        .map(|v| u32::from_str_radix(v, 10).unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let numbers = parse_input("3,4,3,1,2");
        assert_eq!(vec![3, 4, 3, 1, 2], numbers);
    }

    #[test]
    fn test_school_from() {
        let school = SchoolOfFish::from("3,4,3,1,2");
        // starts with a single generation
        assert_eq!(1, school.all_generations.len());
        // there are only 4 entries because there are two 3s
        assert_eq!(4, school.all_generations[&6].len());
        // but when getting the total size we still get 5
        assert_eq!(5, school.len());
    }

    #[test]
    fn test_school_progeny() {
        let mut school = SchoolOfFish::from("3,4,3,1,2");

        // 1
        school.next_day();
        assert_eq!(5, school.len());

        // 2
        school.next_day();
        assert_eq!(6, school.len());

        // 3
        school.next_day();
        assert_eq!(7, school.len());

        // 4
        school.next_day();
        assert_eq!(9, school.len());

        // 5
        school.next_day();
        assert_eq!(10, school.len());

        // 6
        school.next_day();
        assert_eq!(10, school.len());

        // 7
        school.next_day();
        assert_eq!(10, school.len());

        // 8
        school.next_day();
        assert_eq!(10, school.len());

        // 9
        school.next_day();
        assert_eq!(11, school.len());

        // 10
        school.next_day();
        assert_eq!(12, school.len());

        // 11
        school.next_day();
        assert_eq!(15, school.len());

        // 12
        school.next_day();
        assert_eq!(17, school.len());

        // 13
        school.next_day();
        assert_eq!(19, school.len());

        // 14
        school.next_day();
        assert_eq!(20, school.len());

        // 15
        school.next_day();
        assert_eq!(20, school.len());

        // 16
        school.next_day();
        assert_eq!(21, school.len());

        // 17
        school.next_day();
        assert_eq!(22, school.len());

        // 18
        school.next_day();
        assert_eq!(26, school.len());
    }
}