fn check_line(line: &[u8]) -> bool {
    let mut numbers_count: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut has_two_same_adjacent = false;
    let never_descending = line
        .windows(2)
        .filter(|window| {
            has_two_same_adjacent = window[0] == window[1];
            window[0] > window[1]
        }).count() == 0;
    has_two_same_adjacent && never_descending
}

fn number_into_vec(number: i32) -> Vec<u8> {
    number.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

pub fn solve() -> u32 {
    (136818..685979)
        .filter(|number| check_line(&number_into_vec(*number)))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_line() {
        let line: Vec<u8> = vec![1, 1, 1, 1, 1, 1];
        assert!(check_line(&line))
    }

    #[test]
    fn test_check_line_2() {
        let line: Vec<u8> = vec![2, 2, 3, 4, 5, 0];
        assert!(!check_line(&line))
    }

    #[test]
    fn test_check_line_3() {
        let line: Vec<u8> = vec![1, 2, 3, 7, 8, 9];
        assert!(!check_line(&line))
    }

    #[test]
    fn test_number_into_vector() {
        assert_eq!(number_into_vec(111111), vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(number_into_vec(223450), vec![2, 2, 3, 4, 5, 0]);
        assert_eq!(number_into_vec(123789), vec![1, 2, 3, 7, 8, 9]);
    }
}
