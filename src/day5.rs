use crate::intcode;

pub fn solve() -> i64 {
    let filename = "input/day5.txt";
    let mut original_input: Vec<i64> = intcode::read_input(filename);
    let output = intcode::program(&mut original_input, ||5, None);
    1
}


