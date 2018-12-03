use super::get_input_vec;

use std::collections::HashMap;

pub fn problem_two_part_one() -> u32 {
    let input = get_input_vec("two.txt");
    multiply_frequencies_of_two_and_three_duplicates(&input)
}

pub fn problem_two_part_two() -> String {
    let input = get_input_vec("two.txt");
    find_common_letters_of_correct_ids(&input)
}

fn multiply_frequencies_of_two_and_three_duplicates(input: &[String]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    input.iter().for_each(|word| {
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

fn find_common_letters_of_correct_ids(ids: &[String]) -> String {
    let correct_ids = find_correct_ids(ids).expect("There has to be a correct ID in the input");

    correct_ids
        .0
        .chars()
        .zip(correct_ids.1.chars())
        .filter_map(|v| if v.0 == v.1 { Some(v.0) } else { None })
        .collect()
}

fn find_correct_ids(ids: &[String]) -> Option<(&str, &str)> {
    for i in ids {
        for j in ids {
            if differs_by_one_character(i, j) {
                return Some((i, j));
            }
        }
    }

    None
}

fn differs_by_one_character(first: &str, second: &str) -> bool {
    first
        .chars()
        .zip(second.chars())
        .filter(|v| v.0 != v.1)
        .count()
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_common_letters_of_correct_ids() {
        // given
        let input = vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        // that
        let common_letters = find_common_letters_of_correct_ids(&input);

        // then
        assert_eq!("fgij", common_letters);
    }

    #[test]
    fn find_correct_id_returns_none_if_no_matches() {
        // given
        let input = vec![
            "abcd".to_owned(),
            "cfga".to_owned(),
            "pids".to_owned(),
            "pova".to_owned(),
        ];

        // that
        let correct_ids = find_correct_ids(&input);

        // then
        assert!(correct_ids.is_none());
    }

    #[test]
    fn find_correct_id_returns_matching_ids() {
        // given
        let input = vec![
            "cfga".to_owned(),
            "abcd".to_owned(),
            "pids".to_owned(),
            "abud".to_owned(),
        ];

        // that
        let correct_ids = find_correct_ids(&input);

        // then
        assert!(correct_ids.is_some());
        let values = correct_ids.unwrap();
        assert!(values == ("abcd", "abud") || values == ("abud", "abcd"));
    }

    #[test]
    fn differs_by_one_is_true_if_two_different_characters() {
        // given
        let input1 = "a";
        let input2 = "b";

        // that
        let differs_by_one = differs_by_one_character(input1, input2);

        // then
        assert!(differs_by_one);
    }

    #[test]
    fn differs_by_one_is_false_if_two_equal_characters() {
        // given
        let input1 = "a";
        let input2 = "a";

        // that
        let differs_by_one = differs_by_one_character(input1, input2);

        // then
        assert!(!differs_by_one);
    }

    #[test]
    fn multiple_characters_with_one_differing_is_true() {
        // given
        let input1 = "ndlaba";
        let input2 = "ndljba";

        // that
        let differs_by_one = differs_by_one_character(input1, input2);

        // then
        assert!(differs_by_one);
    }

    #[test]
    fn multiple_characters_with_two_differing_is_false() {
        // given
        let input1 = "ndlaba";
        let input2 = "ndwana";

        // that
        let differs_by_one = differs_by_one_character(input1, input2);

        // then
        assert!(!differs_by_one);
    }

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
