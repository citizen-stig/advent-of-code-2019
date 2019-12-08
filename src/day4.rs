fn check_line(line: &[u8]) -> bool {
    let mut matching_digits = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let never_descending = line
        .windows(2)
        .filter(|window| {
            if window[0] == window[1] {
                matching_digits[window[1] as usize] += 1;
            }
            window[0] > window[1]
        })
        .count() == 0;
    let has_proper_matching = matching_digits.iter().filter(|c| **c == 1).count() > 0;
    has_proper_matching && never_descending
}

fn number_into_vec(number: i32) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
}

pub fn solve() -> u32 {
    (136_818..685_979)
        .filter(|number| check_line(&number_into_vec(*number)))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_line {
        use super::*;

        #[test]
        fn test_positive() {
            assert!(check_line(&vec![1, 1, 2, 2, 3, 3]));
            assert!(check_line(&vec![1, 1, 1, 1, 2, 2]));
            assert!(check_line(&vec![1, 1, 2, 3, 4, 5]));
            assert!(check_line(&vec![1, 2, 3, 3, 4, 5]));
            assert!(check_line(&vec![1, 2, 3, 4, 5, 5]));
            assert!(check_line(&vec![1, 2, 3, 4, 5, 5]));
        }

        #[test]
        fn test_negative() {
            assert!(!check_line(&vec![1, 1, 1, 1, 1, 1]));
            assert!(!check_line(&vec![1, 2, 3, 4, 4, 4]));
            assert!(!check_line(&vec![1, 1, 1, 2, 2, 2]));
            assert!(!check_line(&vec![1, 1, 1, 1, 2, 3]));
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
}
