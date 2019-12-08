fn check_line(line: &[u8]) -> bool {
    let mut has_two_same_adjacent = false;
    let never_descending = line
        .windows(2)
        .filter(|window| {
            has_two_same_adjacent = has_two_same_adjacent || window[0] == window[1];
            window[0] > window[1]
        })
        .count()
        == 0;
    has_two_same_adjacent && never_descending
}

fn number_into_vec(number: i32) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

fn check_number(number: i32) -> bool {
    check_line(&number_into_vec(number))
}

pub fn solve() -> u32 {
    (136_818..685_979)
        .filter(|number| check_number(*number))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_line {
        use super::*;

        #[test]
        fn test_positive() {
            assert!(check_line(&vec![1, 1, 1, 1, 1, 1]));
            assert!(check_line(&vec![1, 1, 2, 3, 4, 5]));
            assert!(check_line(&vec![1, 2, 3, 3, 4, 5]));
            assert!(check_line(&vec![1, 2, 3, 4, 5, 5]));
            assert!(check_line(&vec![1, 2, 3, 4, 5, 5]));
            assert!(check_line(&vec![1, 1, 1, 2, 2, 2]));
            assert!(check_line(&vec![1, 1, 1, 1, 2, 3]));
        }

        #[test]
        fn test_negative() {
            assert!(!check_line(&vec![2, 2, 3, 4, 5, 6]));
            assert!(!check_line(&vec![1, 2, 3, 7, 8, 9]));
            assert!(!check_line(&vec![2, 2, 3, 4, 5, 6]));
            assert!(!check_line(&vec![1, 3, 5, 6, 7, 9]));
        }
    }

    #[test]
    fn test_number_into_vector() {
        assert_eq!(number_into_vec(111_111), vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(number_into_vec(223_450), vec![2, 2, 3, 4, 5, 0]);
        assert_eq!(number_into_vec(123_789), vec![1, 2, 3, 7, 8, 9]);
        assert_eq!(number_into_vec(136_818), vec![1, 3, 6, 8, 1, 8]);
        assert_eq!(number_into_vec(685_979), vec![6, 8, 5, 9, 7, 9]);
        assert_eq!(number_into_vec(111_123), vec![1, 1, 1, 1, 2, 3]);
    }

    mod test_check_number {
        use super::*;

        #[test]
        fn test_positive() {
            assert!(check_number(111_111));
            assert!(check_number(111_123));
            assert!(check_number(122_345));
            assert!(check_number(112_345));
            assert!(check_number(111_222));
        }
    }
}
