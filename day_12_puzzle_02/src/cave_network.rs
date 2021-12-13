use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum CaveSize {
    Small,
    Big,
}

impl CaveSize {
    pub fn from(name: &str) -> CaveSize {
        if name.chars().filter(|c| c.is_ascii_uppercase()).count() == name.len() {
            // all uppercase name means LARGE
            CaveSize::Big
        } else {
            // otherwise means small
            CaveSize::Small
        }
    }
}

type CaveRef<'input> = Rc<RefCell<Cave<'input>>>;

pub struct Cave<'input> {
    name: &'input str,
    size: CaveSize,
    connections: Vec<CaveRef<'input>>,
}

impl<'input> Cave<'input> {
    pub fn new(name: &str) -> Rc<RefCell<Cave>> {
        Rc::new(RefCell::new(Cave {
            name,
            size: CaveSize::from(name),
            connections: vec![],
        }))
    }

    pub fn add_connection(&mut self, other_cave: CaveRef<'input>) {
        self.connections.push(other_cave);
    }
}

pub fn find_paths<'input>(network: &HashMap<&str, CaveRef<'input>>, start: CaveRef<'input>, end: CaveRef<'input>) -> Vec<Vec<CaveRef<'input>>> {
    let mut start_to_end_paths: HashMap<String,Vec<CaveRef<'input>>> = HashMap::new();

    for duplicate_allowed in network.values().filter(|c| c.borrow().name != "start" && c.borrow().name != "end" && c.borrow().size == CaveSize::Small) {
        let mut cave_paths: Vec<Vec<CaveRef<'input>>> = vec![];

        cave_paths.push(vec![start.clone()]);

        while cave_paths.len() > 0 {
            let current_path = cave_paths.pop().unwrap();
            for next_cave in current_path.last().unwrap().borrow().connections.iter() {
                let allowed_duplicate_count = if next_cave.borrow().name == duplicate_allowed.borrow().name {
                    1
                } else {
                    0
                };
                if next_cave.borrow().size == CaveSize::Small && current_path.iter().filter(|c| c.borrow().name == next_cave.borrow().name).count() > allowed_duplicate_count  {
                    continue;
                }
                let mut next_path = current_path.clone();
                next_path.push(next_cave.clone());
                if next_cave.borrow().name == end.borrow().name {
                    // because we are looping for each small cave (excluding start and end) we need to exclude duplicates
                    start_to_end_paths.entry(write_path(&next_path)).or_insert(next_path);
                } else {
                    cave_paths.push(next_path);
                }
            }
        }
    }

    start_to_end_paths.into_iter().map(|(_, p)| p.clone()).collect::<Vec<Vec<CaveRef<'input>>>>()
}

fn write_path<'input>(path: &Vec<CaveRef<'input>>) -> String {
    path.iter().map(|c| c.borrow().name).collect::<Vec<&str>>().join(",")
}

fn write_sorted_paths<'input>(paths:&Vec<Vec<CaveRef<'input>>>) -> Vec<String> {
    let mut paths = paths
        .iter()
        .map(|p| write_path(p))
        .collect::<Vec<String>>();
    paths.sort();

    paths
}

pub fn create_cave_network<'input>(connection_inputs: Vec<&'input str>) -> HashMap<&str, CaveRef<'input>> {
    let mut all_caves = HashMap::new();

    for connection_input in connection_inputs.iter() {
        let mut split = connection_input.split("-");
        let start = split.next().unwrap();
        let end = split.next().unwrap();
        if let Some(_) = split.next() {
            panic!("connection inputs should only have two components (start to end) - found extra component(s) in '{}'", connection_input);
        }

        all_caves.entry(start).or_insert(Cave::new(start));
        all_caves.entry(end).or_insert(Cave::new(end));

        let start_cave = all_caves[start].clone();
        let end_cave = all_caves[end].clone();

        start_cave.borrow_mut().add_connection(end_cave.clone());
        end_cave.borrow_mut().add_connection(start_cave.clone());
    }

    all_caves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_demo_1() {
        let network = create_cave_network(vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ]);

        let start_cave = network["start"].clone();
        let end_cave = network["end"].clone();

        let paths = find_paths(&network, start_cave.clone(), end_cave.clone());
        assert_eq!(36, paths.len());
    }

    #[test]
    fn test_create_demo_2() {
        let network = create_cave_network(vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc"
        ]);

        let start_cave = network["start"].clone();
        let end_cave = network["end"].clone();

        let paths = find_paths(&network, start_cave.clone(), end_cave.clone());
        assert_eq!(103, paths.len());
    }

    #[test]
    fn test_create_demo_3() {
        let network = create_cave_network(vec![
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW"
        ]);

        let start_cave = network["start"].clone();
        let end_cave = network["end"].clone();

        let paths = find_paths(&network, start_cave.clone(), end_cave.clone());
        assert_eq!(3509, paths.len());
    }
}