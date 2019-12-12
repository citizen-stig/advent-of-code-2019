use permutohedron::Heap;

use crate::intcode;

pub fn solve() -> i32 {
    let filename = "input/day7.txt";
    let program: Vec<i32> = intcode::read_input(filename);
    find_max_output(&program)
}

fn find_max_output(program: &[i32]) -> i32 {
    let mut max_output = 0;
    let mut configuration = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut configuration);
    while let Some(possible_configuration) = heap.next_permutation() {
        let output = get_thrusters_output(program, &possible_configuration[..]);
        if output > max_output {
            max_output = output
        }
    }

    max_output
}


fn get_thrusters_output(program: &[i32], configuration: &[i32]) -> i32 {
    let mut second_value: i32 = 0;
    for phase in configuration {
        let inputs = vec![*phase, second_value];
        second_value = evaluate_amplifier(program, &inputs);
    }
    second_value
}


fn evaluate_amplifier(program: &[i32], input: &[i32]) -> i32 {
    let mut program = program.clone();
    intcode::program(&mut program.to_owned(), input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thrusters_output_1() {
        let program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let configuration = vec![4, 3, 2, 1, 0];
        assert_eq!(get_thrusters_output(&program, &configuration), 43210)
    }

    #[test]
    fn test_thrusters_output_2() {
        let program = vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
        let configuration = vec![0, 1, 2, 3, 4];
        assert_eq!(get_thrusters_output(&program, &configuration), 54321)
    }

    #[test]
    fn test_thrusters_output_3() {
        let program = vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let configuration = vec![1, 0, 4, 3, 2];
        assert_eq!(get_thrusters_output(&program, &configuration), 65210)
    }

    #[test]
    fn fin_max_output_1() {
        let program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        assert_eq!(find_max_output(&program), 43210)
    }

    #[test]
    fn fin_max_output_2() {
        let program = vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
        assert_eq!(find_max_output(&program), 54321)
    }
}