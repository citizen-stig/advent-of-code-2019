use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct Object {
    name: String,
    on_orbit: Vec<Object>,
}

impl Object {
    pub fn new(name: &str) -> Self {
        Object { name: name.to_owned(), on_orbit: Vec::new() }
    }

    pub fn add_to_orbit(&mut self, object: Object) {
        self.on_orbit.push(object);
    }
}


fn count_orbits(lines: &Vec<String>) -> usize {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let names: Vec<String> = line.split(")").map(|s| s.to_owned()).collect();
        let children = match map.entry(names[0].to_owned()) {
            Vacant(entry) => {
                entry.insert(Vec::<String>::new())
            },
            Occupied(entry) => {
                entry.into_mut()
            }
        };
        children.push(names[1].to_owned())
    }
    println!("MAP: {:?}", map);
    println!("Iterating...");
    let mut queue: Vec<String> = vec!["COM".to_owned()];
    let mut count = 0;
    let mut extra = 0;
    while !queue.is_empty() {
        let current = queue.remove(0);
        match map.get_mut(&current) {
            Some(children) => {
                count += children.len() + extra;
                queue.append(children);
            },
            None => (),
        }
        extra += 1
    }
    println!("Count: {:?}", count);
    count
}

pub fn solve() -> usize {
    let filename = "input/day6.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    count_orbits(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let lines = vec!["COM)B".to_owned()];

        assert_eq!(count_orbits(&lines), 1);
    }

    #[test]
    fn test_two() {
        let lines = vec!["COM)B".to_owned(), "COM)C".to_owned()];

        assert_eq!(count_orbits(&lines), 2);
    }

    #[test]
    fn test_tree() {
        let lines = vec!["COM)B", "B)C"].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 3);
    }

    #[test]
    #[ignore]
    fn test_example() {
        let lines = vec!["COM)B",
                         "B)C",
                         "C)D",
                         "D)E",
                         "E)F",
                         "B)G",
                         "G)H",
                         "D)I",
                         "E)J",
                         "J)K",
                         "K)L", ].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 42);
    }
}