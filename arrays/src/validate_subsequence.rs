pub mod arrays {

    //Time : O(n) , Space : O(1)
    pub fn validate_subsequence<T>(input: &[T], sequence: &[T]) -> bool
    where
        T: PartialEq,
    {
        let mut index = 0;
        for data in input.iter() {
            if index == sequence.len() {
                break;
            }

            if sequence[index] == *data {
                index += 1;
            }
        }

        index == sequence.len()
    }
}

#[cfg(test)]
mod arrays_tests {
    use super::arrays::*;

    #[test]
    fn test_validate_subsequence() {
        let input = [4, 6, 1];
        let subsequence = [6, 4];
        let result = validate_subsequence(&input, &subsequence);
        assert_eq!(result, false);

        let input = [1, 1, 6, 1];
        let subsequence = [1, 1, 1];
        let result = validate_subsequence(&input, &subsequence);
        assert_eq!(result, true);

        let input = [5, 1, 22, 25, 6, -1, 8, 10];
        let subsequence = [1, 6, -1, 5];
        let result = validate_subsequence(&input, &subsequence);
        assert_eq!(result, false);

        let input = [5, 1, 22, 25, 6, -1, 8, 10];
        let subsequence = [26];
        let result = validate_subsequence(&input, &subsequence);
        assert_eq!(result, false);
    }
}
