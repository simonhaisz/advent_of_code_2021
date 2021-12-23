use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use crate::cave::{Cave, Position};

pub fn find_safest_path<'input>(cave: &'input Cave, start: &'input Position, end: &'input Position) -> Vec<&'input Position> {
    let mut frontier = BinaryHeap::new();
    frontier.push(PriorityPos::new(start, 0));

    let mut came_from: HashMap<&Position, &Position> = HashMap::new();
    let mut risk_so_far: HashMap<&Position, u32> = HashMap::new();
    risk_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.pos == end {
            break;
        }

        for next in cave.neighbors(current.pos) {
            let new_risk = risk_so_far[current.pos] + next.risk();
            if !risk_so_far.contains_key(next) || new_risk < risk_so_far[next] {
                risk_so_far.insert(next, new_risk);
                let priority = new_risk as usize + current.pos.distance(next);
                frontier.push(PriorityPos::new(next, priority));
                came_from.insert(next, current.pos);
            }
        }
    }

    let mut path = vec![end];
    let mut current = end;
    while current != start {
        let prev = came_from[current];
        path.insert(0, prev);
        current = prev;
    }

    path
}

struct PriorityPos<'input> {
    pos: &'input Position,
    priority: usize,
}

impl<'input> PriorityPos<'input> {
    fn new(pos: &'input Position, priority: usize) -> PriorityPos<'input> {
        PriorityPos {
            pos,
            priority,
        }
    }
}

impl<'input> PartialEq for PriorityPos<'input> {

    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<'input> Eq for PriorityPos<'input> {}

impl<'input> Ord for PriorityPos<'input> {

    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<'input> PartialOrd for PriorityPos<'input> {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    use crate::cave::{CaveBuilder};

	#[test]
	fn test_demo() {
		let rows = "
		1163751742
		1381373672
		2136511328
		3694931569
		7463417111
		1319128137
		1359912421
		3125421639
		1293138521
		2311944581"
			.trim()
            .split("\n")
            .map(|r| r.trim().chars()
                .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>();

		let mut builder = CaveBuilder::new();
		for row in rows.into_iter() {
			builder.add_row(row);
		}

		let cave = builder.build();

		let path = find_safest_path(&cave, cave.start(), cave.end());

        assert_eq!(19, path.len());

        let total_risk: u32 = path
            .iter()
            .map(|p| if *p == cave.start() { 0 } else { p.risk() })
            .sum();

		assert_eq!(40, total_risk);
	}
}