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

pub struct CaveNetwork<'input> {
    caves: HashMap<&'input str, CaveRef<'input>>,
}

impl <'input> CaveNetwork<'input> {
    pub fn new() -> CaveNetwork<'input> {
        CaveNetwork {
            caves: HashMap::new(),
        }
    }

    pub fn add_cave(&mut self, cave: CaveRef<'input>) {
        self.caves.insert(cave.borrow().name, cave.clone());
    }

    // pub fn get_cave_mut(&mut self, name: &str) -> Option<&mut Cave<'input>> {
    //     self.caves.get_mut(name)
    // }

    pub fn find_a_path(&self, start: &str, end: &str) -> Vec<CaveRef<'input>> {
        if let Some(start) = self.caves.get(start) {
            let mut cave_paths = vec![];



            cave_paths
        } else {
            panic!("There is no cave with name '{}'", start);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_1() {
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
}