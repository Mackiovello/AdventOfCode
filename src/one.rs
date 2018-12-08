use std::collections::HashSet;

use super::get_int_input_vec;

pub fn problem_one_part_one() -> i32 {
    let numbers = get_int_input_vec("one.txt");
    numbers.iter().sum()
}

pub fn problem_one_part_two() -> i32 {
    let numbers = get_int_input_vec("one.txt");

    find_first_duplicated_frequency(&numbers)
}

fn find_first_duplicated_frequency(numbers: &[i32]) -> i32 {
    let mut unique_elements = HashSet::new();

    unique_elements.insert(0);

    let mut resulting_frequencies = get_resulting_frequencies(0, &numbers);

    loop {
        for x in &resulting_frequencies {
            if !unique_elements.insert(*x) {
                return *x;
            }
        }

        let last = resulting_frequencies.last().unwrap();
        resulting_frequencies = get_resulting_frequencies(*last, &numbers);
    }
}

fn get_resulting_frequencies(initial: i32, numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .scan(initial, |state, &n| {
            *state += n;
            Some(*state)
        })
        .collect::<Vec<i32>>()
}
