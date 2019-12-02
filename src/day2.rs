use std::fs;

fn program(input: Vec<i32>) -> Vec<i32> {
    let mut data = input.clone();
    let mut position = 0;
    loop {
        let opcode = data[position];
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
            99 => break,
            _ => panic!("Unknown opcode {}", opcode),
        }
    }
    data
}

pub fn solve() -> Option<i32> {
    let filename = "input/day2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let original_input: Vec<i32> = contents
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let mut answer = None;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = try_input(&original_input, noun, verb);
            if result == 19_690_720 {
                answer = Some(100 * noun + verb);
                break;
            }
        }
    }
    answer
}

fn try_input(data: &[i32], noun: i32, verb: i32) -> i32 {
    let mut try_data = data.to_owned();
    try_data[1] = noun;
    try_data[2] = verb;
    let output = program(try_data);
    output[0]
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
}
