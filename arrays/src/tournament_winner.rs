pub mod arrays {

    //Time : O(n) - Single Iteration over matches
    //Space : O(n) Using hashmap to store scores
    pub fn tour_winner<'a>(matches: Vec<[&'a str; 2]>, results: Vec<u32>) -> &'a str {
        use std::collections::HashMap;

        let mut records = HashMap::new();
        let mut tour_winner: &str = "";
        for (index, _tuple) in matches.iter().enumerate() {
            let curr_winner: &str;
            curr_winner = matches.get(index).unwrap()[1 - (results[index] as usize)];

            let val = records.entry(curr_winner).or_insert(0);
            *val += 3;

            if tour_winner.is_empty() || records.get(&curr_winner).unwrap() > records.get(&tour_winner).unwrap() {
                tour_winner = curr_winner;
            }
        }
        tour_winner
    }
}

#[cfg(test)]
mod arrays_tests {
    use super::arrays::*;

    #[test]
    fn test_tour_winner() {
        let input = vec![["HTML", "Java"], ["Java", "Python"], ["Python", "HTML"]];
        let result = tour_winner(input, vec![0, 1, 1]);
        assert_eq!(result, "Java");

        let input = vec![["HTML", "Java"], ["Java", "Python"], ["Python", "HTML"]];
        let result = tour_winner(input, vec![0, 1, 1]);
        assert_eq!(result, "Java");

        let input = vec![["A", "B"]];
        let result = tour_winner(input, vec![0]);
        assert_eq!(result, "B");

        let input = vec![["Bulls", "Eagles"], ["Bulls", "Bears"], ["Bears", "Eagles"]];
        let result = tour_winner(input, vec![0, 0, 0]);
        assert_eq!(result, "Eagles");

        let input = vec![
            ["Bulls", "Eagles"],
            ["Bulls", "Bears"],
            ["Bulls", "Monkeys"],
            ["Eagles", "Bears"],
            ["Eagles", "Monkeys"],
            ["Bears", "Monkeys"],
        ];
        let result = tour_winner(input, vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(result, "Bulls");
    }
}
