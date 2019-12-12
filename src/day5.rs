use crate::intcode;

pub fn solve() -> i32 {
    let filename = "input/day5.txt";
    let mut original_input: Vec<i32> = intcode::read_input(filename);
    let output = intcode::program(&mut original_input, &vec![5]);
    output
}


