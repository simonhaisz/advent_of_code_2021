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

fn find_paths<'input>(start: CaveRef<'input>, end: CaveRef<'input>) -> Vec<Vec<CaveRef<'input>>> {
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

fn create_cave_network<'input>(connection_inputs: Vec<&'input str>) -> HashMap<&str, CaveRef<'input>> {
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
}