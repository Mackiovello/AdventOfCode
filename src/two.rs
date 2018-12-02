use super::get_input_vec;

use std::collections::HashMap;

pub fn problem_two_part_one() -> u32 {
    let input = get_input_vec("two.txt");
    multiply_frequencies_of_two_and_three_duplicates(&input)
}

fn multiply_frequencies_of_two_and_three_duplicates(input: &[String]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    input.iter().for_each(|word| {
        println!("{}", word);
        if has_number_of_equal_characters(word, 2) {
            twos += 1;
        }
        if has_number_of_equal_characters(word, 3) {
            threes += 1;
        }
    });

    twos * threes
}

fn has_number_of_equal_characters(id: &str, frequency: u32) -> bool {
    let mut frequencies = HashMap::new();

    id.chars().for_each(|c| {
        let v = frequencies.entry(c).or_insert(0);
        *v += 1;
    });

    frequencies.values().any(|v: &u32| *v == frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_passes() {
        // given
        let input = vec![
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned(),
        ];

        // that
        let result = multiply_frequencies_of_two_and_three_duplicates(&input);

        // then
        assert_eq!(12, result);
    }

    #[test]
    fn finds_four_equal_values() {
        // given
        let input = "9khjl9vsd9da9";

        // that
        let has_four_equal_chars = has_number_of_equal_characters(input, 4);

        // then
        assert!(has_four_equal_chars);
    }

    #[test]
    fn finds_if_there_are_only_two_equal_numbers() {
        // given
        let input = "99";

        // that
        let has_two_equal_chars = has_number_of_equal_characters(input, 2);

        // then
        assert!(has_two_equal_chars);
    }

    #[test]
    fn is_false_if_two_numbers_are_not_equal() {
        // given
        let input = "79";

        // that
        let has_two_equal_chars = has_number_of_equal_characters(input, 2);

        // then
        assert!(!has_two_equal_chars);
    }

    #[test]
    fn finds_three_equal_characters() {
        // given
        let input = "9329dacaosn9";

        // that
        let has_three_equal_chars = has_number_of_equal_characters(input, 3);

        // then
        assert!(has_three_equal_chars);
    }
}
