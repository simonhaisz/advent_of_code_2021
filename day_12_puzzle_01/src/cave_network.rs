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

pub fn find_paths<'input>(start: CaveRef<'input>, end: CaveRef<'input>) -> Vec<Vec<CaveRef<'input>>> {
    let mut start_to_end_paths: Vec<Vec<CaveRef<'input>>> = vec![];

    let mut cave_paths: Vec<Vec<CaveRef<'input>>> = vec![];

    cave_paths.push(vec![start.clone()]);

    while cave_paths.len() > 0 {
        let current_path = cave_paths.pop().unwrap();
        for next_cave in current_path.last().unwrap().borrow().connections.iter() {
            if next_cave.borrow().size == CaveSize::Small && current_path.iter().any(|c| c.borrow().name == next_cave.borrow().name) {
                continue;
            }
            let mut next_path = current_path.clone();
            next_path.push(next_cave.clone());
            if next_cave.borrow().name == end.borrow().name {
                start_to_end_paths.push(next_path);
            } else {
                cave_paths.push(next_path);
            }
        }
    }

    start_to_end_paths
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

    #[test]
    fn test_path_demo_1() {
        let cave_start = Cave::new("start");
        let cave_a = Cave::new("A");
        let cave_b = Cave::new("b");
        let cave_c = Cave::new("c");
        let cave_d = Cave::new("d");
        let cave_end = Cave::new("end");

        {
            let mut mut_start = cave_start.borrow_mut();
            mut_start.add_connection(cave_a.clone());
            mut_start.add_connection(cave_b.clone());
        }

        {
            let mut mut_a = cave_a.borrow_mut();
            mut_a.add_connection(cave_start.clone());
            mut_a.add_connection(cave_c.clone());
            mut_a.add_connection(cave_b.clone());
            mut_a.add_connection(cave_end.clone());
        }

        {
            let mut mut_b = cave_b.borrow_mut();
            mut_b.add_connection(cave_start.clone());
            mut_b.add_connection(cave_a.clone());
            mut_b.add_connection(cave_d.clone());
            mut_b.add_connection(cave_end.clone());
        }

        {
            let mut mut_d = cave_d.borrow_mut();
            mut_d.add_connection(cave_b.clone());
        }

        {
            let mut mut_c = cave_c.borrow_mut();
            mut_c.add_connection(cave_a.clone());
        }

        {
            let mut mut_end = cave_end.borrow_mut();
            mut_end.add_connection(cave_a.clone());
            mut_end.add_connection(cave_b.clone());
        }

        let paths = find_paths(cave_start.clone(), cave_end.clone());
        assert_eq!(10, paths.len());
        assert_eq!(vec![
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,end"
            ],
            write_sorted_paths(&paths)
        );
    }

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

        let paths = find_paths(start_cave.clone(), end_cave.clone());
        assert_eq!(10, paths.len());
        assert_eq!(vec![
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,end"
            ],
            write_sorted_paths(&paths)
        );
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

        let paths = find_paths(start_cave.clone(), end_cave.clone());
        assert_eq!(19, paths.len());
        assert_eq!(vec![
            "start,HN,dc,HN,end",
            "start,HN,dc,HN,kj,HN,end",
            "start,HN,dc,end",
            "start,HN,dc,kj,HN,end",
            "start,HN,end",
            "start,HN,kj,HN,dc,HN,end",
            "start,HN,kj,HN,dc,end",
            "start,HN,kj,HN,end",
            "start,HN,kj,dc,HN,end",
            "start,HN,kj,dc,end",
            "start,dc,HN,end",
            "start,dc,HN,kj,HN,end",
            "start,dc,end",
            "start,dc,kj,HN,end",
            "start,kj,HN,dc,HN,end",
            "start,kj,HN,dc,end",
            "start,kj,HN,end",
            "start,kj,dc,HN,end",
            "start,kj,dc,end"
            ],
            write_sorted_paths(&paths)
        );
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

        let paths = find_paths(start_cave.clone(), end_cave.clone());
        assert_eq!(226, paths.len());
    }
}