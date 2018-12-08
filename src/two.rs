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
