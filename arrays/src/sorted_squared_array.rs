pub mod arrays {

    //T(n) : O(n) , Space S(n) : O(n) for output vector  
    pub fn sorted_squared_array(input: &[i32]) -> Vec<i32> {
        let length = input.len();

        if length == 0 {
            eprintln!("Array passed is empty");
        }

        let mut left = 0;
        let mut right = length - 1;        

        //let output  = input.iter().map(|_| 0).collect::<Vec<i32>>();
        let mut output: Vec<i32> = vec![0; input.len()]; // input.to_vec(), but if data is huge , then output size is wasted

        let mut index = length - 1;
        while left < right {
            if input[left].abs() <= input[right].abs() {
                output[index] = input[right].pow(2);
                right -= 1;
            } else {
                output[index] = input[left].pow(2);
                left += 1;
            }
            index -= 1;
        }

        output[index] = input[left].pow(2);
        output             
    }
}

#[cfg(test)]
pub mod arrays_tests {

    use super::arrays::*;
    #[test]
    fn test_sorted_squared_array() {

        let input = [1, 4, 6];
        let result = sorted_squared_array(&input);
        assert_eq!(result, [1 , 16, 36]);

        let input = [0];
        let result = sorted_squared_array(&input);
        assert_eq!(result, [0]);

        let input = [-2, -1];
        let result = sorted_squared_array(&input);
        assert_eq!(result, [1 ,4]);

        let input = [-3, -2, 0, 1];
        let result = sorted_squared_array(&input);
        assert_eq!(result, [0, 1, 4, 9]);

        let input = [-10, -5, 0, 5, 10];
        let result = sorted_squared_array(&input);
        assert_eq!(result, [0, 25, 25, 100, 100]);
    }
}
