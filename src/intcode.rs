enum Mode {
    Position,
    Immediate
}


pub fn program(input: Vec<i32>) -> Vec<i32> {
    let mut data = input.clone();
    let mut position = 0;
    loop {
        let instruction = data[position];


        match opcode {
            1 | 2 => {
                let left = data[data[position + 1] as usize];
                let right = data[data[position + 2] as usize];
                let destination = data[position + 3] as usize;
                data[destination] = if opcode == 1 {
                    left + right
                } else {
                    left * right
                };
                position += 4;
            }
            3 => {}
            99 => break,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let input = vec![1, 0, 0, 0, 99];
        let expected_output = vec![2, 0, 0, 0, 99];
        assert_eq!(program(input), expected_output)
    }

    #[test]
    fn test_multiply() {
        let input = vec![2, 3, 0, 3, 99];
        let expected_output = vec![2, 3, 0, 6, 99];
        assert_eq!(program(input), expected_output)
    }

    #[test]
    fn test_store_after() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let expected_output = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(program(input), expected_output)
    }

    #[test]
    fn test_two_operations() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected_output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(program(input), expected_output)
    }

    #[test]
    fn test_just_with_input() {
        let input = vec![3, 0, 4, 0, 99];
    }
}