pub struct OctopusGrid {
    energy_levels: Vec<u32>,
    size: Option<usize>,
}

impl OctopusGrid {
    pub fn new() -> OctopusGrid {
        OctopusGrid {
            energy_levels: vec![],
            size: None,
        }
    }

    pub fn add_row(&mut self, mut levels: Vec<u32>) {
        if let Some(size) = self.size {
            if levels.len() != size {
                panic!("Expect row to be of size {} - found {}", size, levels.len());
            }
        } else {
            self.size = Some(levels.len());
        }

        self.energy_levels.append(&mut levels);
    }

    pub fn compute_flashes(&self, step_count: u32) -> u32 {
        let mut energy_levels_buffer = self.energy_levels.to_vec();

        let mut flash_count = 0;

        for _ in 0..step_count {
            let mut has_flashed_octopi = vec![];
            let mut just_flash_octopi = vec![];
            for (octopus, energy) in energy_levels_buffer.iter_mut().enumerate() {
                *energy += 1;
                if *energy > 9 {
                    has_flashed_octopi.push(octopus);
                    just_flash_octopi.push(octopus);
                    flash_count += 1;
                }
            }
            while just_flash_octopi.len() > 0 {
                let mut newly_flashed_octopi = vec![];

                for flashed_octopus in just_flash_octopi.iter() {
                    for adjacent_octopus in self.adjacent_indicies(*flashed_octopus).iter() {
                        let energy = energy_levels_buffer.get_mut(*adjacent_octopus).unwrap();
                        *energy += 1;
                        if *energy > 9 && !has_flashed_octopi.contains(adjacent_octopus) {
                            has_flashed_octopi.push(*adjacent_octopus);
                            newly_flashed_octopi.push(*adjacent_octopus);
                            flash_count += 1;
                        }
                    }
                }

                just_flash_octopi = newly_flashed_octopi;
            }

            for has_flashed in has_flashed_octopi.iter() {
                let energy = energy_levels_buffer.get_mut(*has_flashed).unwrap();
                *energy = 0;
            }
        }

        flash_count
    }

    fn adjacent_indicies(&self, center: usize) -> Vec<usize> {
        let mut indicies = vec![];

        let size = self.size.unwrap();

        let row = (center / size) as isize;
        let column = (center % size) as isize;

        for x_delta in -1..=1 {
            for y_delta in -1..=1 {
                let x = column + x_delta;
                let y = row + y_delta;
                if x >= 0 && (x as usize) < size && y >= 0 && (y as usize) < size {
                    let index = (x as usize) + (y as usize) * size;
                    indicies.push(index);
                }
            }
        }

        indicies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let mut grid = OctopusGrid::new();
        let rows =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .split("\n")
            .map(|r| r.chars()
                .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>();

        for row in rows.into_iter() {
            grid.add_row(row);
        }

        assert_eq!(0, grid.compute_flashes(1));
        assert_eq!(204, grid.compute_flashes(10));
        assert_eq!(1656, grid.compute_flashes(100));
    }
}