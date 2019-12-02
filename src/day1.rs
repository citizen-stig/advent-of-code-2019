use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> i32 {
    let filename = "input/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .map(find_fuel_requirements)
        .sum()
}

fn find_fuel_requirements(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel + find_fuel_requirements(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(find_fuel_requirements(12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(find_fuel_requirements(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(find_fuel_requirements(1969), 966);
    }

    #[test]
    fn test_100756() {
        assert_eq!(find_fuel_requirements(100756), 50346);
    }
}
