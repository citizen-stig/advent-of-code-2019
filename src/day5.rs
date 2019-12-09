use std::fs;

use crate::intcode;

pub fn solve() -> i32 {
    let filename = "input/day5.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut original_input: Vec<i32> = contents
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    let output = intcode::program(&mut original_input, 5);
    output
}


