use permutohedron::Heap;

use crate::intcode;

pub fn solve() -> i64 {
    let filename = "input/day7.txt";
    let program: Vec<i64> = intcode::read_input(filename);
    find_max_output(&program)
}

fn find_max_output(program: &[i64]) -> i64 {
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


fn get_thrusters_output(program: &[i64], configuration: &[i64]) -> i64 {
    let mut last_value: i64 = 0;
    let mut programs: Vec<Vec<i64>> = Vec::new();
    let mut inputs: Vec<i64> = Vec::new();
    let mut positions: Vec<Option<usize>> = Vec::new();
    for _ in 0..configuration.len() {
        programs.push(program.to_vec());
        inputs.push(0);
        positions.push(None);
    }
    let mut index = 0;
    let mut loop_count = 0;
    loop {
        if index == 0 {
            println!("Loop {:?}", loop_count);
            loop_count += 1;
        }
        let mut input_call_count = 0;
        let input = || {
            if input_call_count == 0 {
                input_call_count += 1;
                return configuration[index];
            }
            inputs[index]
        };
        let current_program = &mut programs[index];
        let current_position = positions[index];
        let output = intcode::program(current_program, input, current_position);
        let next_index = (index + 1) % 5;
        match output {
            intcode::ProgramResult::Halt => break,
            intcode::ProgramResult::Output(value, position) => {
                last_value = value;
                inputs[next_index] = value;
                positions[index] = Some(position);
            }
        }
        index = next_index;
    }
    last_value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thrusters_output_1() {
        let program = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        let configuration = vec![9, 8, 7, 6, 5];
        assert_eq!(get_thrusters_output(&program, &configuration), 139629729)
    }

//    #[test]
//    fn test_thrusters_output_2() {
//        let program = vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
//        let configuration = vec![0, 1, 2, 3, 4];
//        assert_eq!(get_thrusters_output(&program, &configuration), 54321)
//    }
//
//    #[test]
//    fn test_thrusters_output_3() {
//        let program = vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
//        let configuration = vec![1, 0, 4, 3, 2];
//        assert_eq!(get_thrusters_output(&program, &configuration), 65210)
//    }
//
//    #[test]
//    fn fin_max_output_1() {
//        let program = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
//        assert_eq!(find_max_output(&program), 43210)
//    }
//
//    #[test]
//    fn fin_max_output_2() {
//        let program = vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
//        assert_eq!(find_max_output(&program), 54321)
//    }
}