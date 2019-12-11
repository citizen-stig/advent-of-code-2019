use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};


fn count_orbits(lines: &Vec<String>) -> usize {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let names: Vec<String> = line.split(")").map(|s| s.to_owned()).collect();
        let children = match map.entry(names[0].to_owned()) {
            Vacant(entry) => {
                entry.insert(Vec::<String>::new())
            }
            Occupied(entry) => {
                entry.into_mut()
            }
        };
        children.push(names[1].to_owned())
    }
    let mut queue: Vec<Option<String>> = vec![Some("COM".to_owned()), None];

    let mut count = 0;
    let mut distance_to_root = 1;
    while queue.len() > 1 {
        let current = queue.remove(0);
        match current {
            Some(current_name) => {
                match map.get_mut(&current_name) {
                    Some(children) => {
                        count += children.len() * distance_to_root;
                        children.iter().for_each(|s| queue.push(Some(s.clone())));
                    }
                    None => (),
                }
            },
            None => {
                distance_to_root += 1;
                queue.push(None)
            }
        }

    }
    count
}

pub fn solve() -> usize {
    let filename = "input/day6.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
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
        let lines = vec!["COM)B", "COM)C"].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 2);
    }

    #[test]
    fn test_tree() {
        let lines = vec!["COM)B", "B)C"].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 3);
    }

    #[test]
    fn test_four() {
        let lines = vec![
            "COM)B",
            "B)C",
            "COM)D"
        ].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 4);
    }

    #[test]
    fn test_seven() {
        let lines = vec![
            "COM)B",
            "B)C",
            "B)D"
        ].iter().map(|s| String::from(*s)).collect();

        assert_eq!(count_orbits(&lines), 5);
    }

    #[test]
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