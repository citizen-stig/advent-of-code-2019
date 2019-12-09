#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum OpCode {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Save(Mode),
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
    Halt,
}


fn instruction_to_op_code(instruction: i32) -> OpCode {
    let operation_code = instruction % 100;
    let params = instruction / 100;
    let match_mode = |param: i32| {
        match param {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode for parameter 1: {:?}", params),
        }
    };
    match operation_code {
        1 | 2 | 5 | 6 | 7 | 8 => {
            let mode_1 = match_mode(params % 10);
            let mode_2 = match_mode(params / 10);
            if operation_code == 1 {
                OpCode::Add(mode_1, mode_2)
            } else if operation_code == 2 {
                OpCode::Multiply(mode_1, mode_2)
            } else if operation_code == 5 {
                OpCode::JumpIfTrue(mode_1, mode_2)
            } else if operation_code == 6 {
                OpCode::JumpIfFalse(mode_1, mode_2)
            } else if operation_code == 7 {
                OpCode::LessThan(mode_1, mode_2)
            } else {
                OpCode::Equals(mode_1, mode_2)
            }
        }
        3 | 4 => {
            let mode = match_mode(params);
            if operation_code == 3 {
                OpCode::Save(mode)
            } else {
                OpCode::Output(mode)
            }
        }
        99 => OpCode::Halt,
        _ => panic!("Unknown operation code {}", operation_code),
    }
}

fn get_actual_value(data: &[i32], mode: Mode, position: usize) -> i32 {
    match mode {
        Mode::Position => data[data[position] as usize].clone(),
        Mode::Immediate => data[position].clone(),
    }
}

pub fn program(data: &mut [i32], input: i32) -> i32 {
    let mut output = 0;
    let mut position = 0;

    loop {
        let operation_code = instruction_to_op_code(data[position]);
        match operation_code {
            OpCode::Add(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                let right = get_actual_value(&data, mode_2, position + 2);
                let destination = data[position + 3] as usize;
                data[destination] = left + right;
                position += 4;
            }
            OpCode::Multiply(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                let right = get_actual_value(&data, mode_2, position + 2);
                let destination = data[position + 3] as usize;
                data[destination] = left * right;
                position += 4;
            }
            OpCode::LessThan(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                let right = get_actual_value(&data, mode_2, position + 2);
                let destination = data[position + 3] as usize;
                data[destination] = if left < right { 1 } else { 0 };
                position += 4
            }
            OpCode::JumpIfTrue(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                if left != 0 {
                    position = get_actual_value(&data, mode_2, position + 2) as usize;
                } else {
                    position += 3
                }
            }
            OpCode::JumpIfFalse(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                if left == 0 {
                    position = get_actual_value(&data, mode_2, position + 2) as usize;
                } else {
                    position += 3
                }
            }
            OpCode::Equals(mode_1, mode_2) => {
                let left = get_actual_value(&data, mode_1, position + 1);
                let right = get_actual_value(&data, mode_2, position + 2);
                let destination = data[position + 3] as usize;
                data[destination] = if left == right { 1 } else { 0 };
                position += 4
            }
            OpCode::Save(_) => {
                let destination = data[position + 1] as usize;
                data[destination] = input;
                position += 2
            }
            OpCode::Output(mode) => {
                output = get_actual_value(&data, mode, position + 1);
                position += 2
            }
            OpCode::Halt => break,
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    mod instruction_to_op_code {
        use super::*;

        #[test]
        fn test_halt() {
            assert_eq!(instruction_to_op_code(99), OpCode::Halt);
        }

        #[test]
        fn test_sum() {
            assert_eq!(instruction_to_op_code(1), OpCode::Add(Mode::Position, Mode::Position));
            assert_eq!(instruction_to_op_code(1001), OpCode::Add(Mode::Position, Mode::Immediate));
            assert_eq!(instruction_to_op_code(101), OpCode::Add(Mode::Immediate, Mode::Position));
            assert_eq!(instruction_to_op_code(1101), OpCode::Add(Mode::Immediate, Mode::Immediate));
        }

        #[test]
        fn test_multiply() {
            assert_eq!(instruction_to_op_code(2), OpCode::Multiply(Mode::Position, Mode::Position));
            assert_eq!(instruction_to_op_code(1002), OpCode::Multiply(Mode::Position, Mode::Immediate));
            assert_eq!(instruction_to_op_code(102), OpCode::Multiply(Mode::Immediate, Mode::Position));
            assert_eq!(instruction_to_op_code(1102), OpCode::Multiply(Mode::Immediate, Mode::Immediate));
        }

        #[test]
        fn test_save() {
            assert_eq!(instruction_to_op_code(3), OpCode::Save(Mode::Position));
            assert_eq!(instruction_to_op_code(103), OpCode::Save(Mode::Immediate));
        }

        fn test_output() {
            assert_eq!(instruction_to_op_code(4), OpCode::Output(Mode::Position));
            assert_eq!(instruction_to_op_code(104), OpCode::Output(Mode::Immediate));
        }
    }

    #[test]
    fn test_sum() {
        let mut data = vec![1, 0, 0, 0, 99];
        let expected_output = vec![2, 0, 0, 0, 99];
        let output = program(&mut data, 0);
        assert_eq!(data, expected_output)
    }

    #[test]
    fn test_multiply() {
        let mut data = vec![2, 3, 0, 3, 99];
        let expected_output = vec![2, 3, 0, 6, 99];
        program(&mut data, 0);
        assert_eq!(data, expected_output)
    }

    #[test]
    fn test_store_after() {
        let mut data = vec![2, 4, 4, 5, 99, 0];
        let expected_output = vec![2, 4, 4, 5, 99, 9801];
        program(&mut data, 0);
        assert_eq!(data, expected_output)
    }

    #[test]
    fn test_two_operations() {
        let mut data = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected_output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        program(&mut data, 0);
        assert_eq!(data, expected_output)
    }

    #[test]
    fn test_just_with_input() {
        let mut data = vec![3, 0, 4, 0, 99];
        let input = 31337;
        let output = program(&mut data, input);
        assert_eq!(input, output);
    }
}