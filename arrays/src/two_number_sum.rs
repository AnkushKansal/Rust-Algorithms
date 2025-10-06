#![allow(non_snake_case)]
pub mod arrays {

    use std::ops;
    //Time : O(n^2), Space : O(1)
    pub fn two_number_sum_non_optimal<T>(input: &[T], target: &T) -> Vec<(T, T)>
    where
        T: ops::Add<Output = T> + PartialEq + Copy,
    {
        let mut out = Vec::new();
        for (index, ele1) in input.iter().enumerate() {
            for ele2 in input.iter().skip(index + 1) {
                if *target == *ele1 + *ele2 {
                    // Add takes ownership
                    out.push((*ele1, *ele2));
                }
            }
        }
        out
    }

    use std::hash::Hash;
    //Time : O(n^2), Space : O(n)
    pub fn two_number_sum_optimal<T>(input: &[T], target: &T) -> Vec<(T, T)>
    where
        T: ops::Sub<Output = T> + Copy + Eq + Hash,
    {
        use std::collections::HashSet;

        let mut out = Vec::new();
        let mut storage = HashSet::new();
        for ele1 in input.iter() {
            if storage.contains(&(*target - *ele1)) {
                out.push((*target - *ele1, *ele1));
            } else {
                storage.insert(ele1);
            }
        }
        out
    }
}

#[cfg(test)]
mod arrays_tests {
    use super::arrays::*;

    #[test]
    fn test_two_number_sum_non_optimal() {
        let input = [3, 5, -4, 8, 11, 1, -1, 6, 4];
        let result = two_number_sum_non_optimal(&input, &10);
        assert_eq!(result, vec![(11, -1), (6, 4)]);

        let input = [4, 6];
        let result = two_number_sum_non_optimal(&input, &10);
        assert_eq!(result, vec![(4, 6)]);

        let input = [4, 6, 1];
        let result = two_number_sum_non_optimal(&input, &5);
        assert_eq!(result, vec![(4, 1)]);

        let input = [15];
        let result = two_number_sum_non_optimal(&input, &15);
        assert_ne!(result, vec![(15, 0)]);

        let input = [3, 5, -4, 8, 11, 1, -1, 6, 4];
        let result = two_number_sum_optimal(&input, &10);
        assert_eq!(result, vec![(11, -1), (6, 4)]);
    }

    #[test]
    fn test_two_number_sum_optimal() {
        let input = [4, 6, 1];
        let result = two_number_sum_optimal(&input, &5);
        assert_eq!(result, vec![(4, 1)]);

        let input = [15];
        let result = two_number_sum_optimal(&input, &15);
        assert_eq!(result, vec![]);

        let input = [3, 5, -4, 8, 11, 1, -1, 6, 4];
        let result = two_number_sum_optimal(&input, &10);
        assert_eq!(result, vec![(11, -1), (6, 4)]);
    }
}
