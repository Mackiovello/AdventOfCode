use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn problem_one_part_one() -> i32 {
    let numbers = get_input_vec();
    numbers.iter().sum()
}

pub fn problem_one_part_two() -> i32 {
    let numbers = get_input_vec();

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

fn get_input_vec() -> Vec<i32> {
    // This is some dumpster fire error handling
    let file_name = "input/one.txt";
    let path = Path::new(file_name);
    let mut file = File::open(&path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    let numbers = content
        .split_whitespace()
        .map(|c| c.parse::<i32>().unwrap());

    numbers.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = vec![3, 3, 4, -2, -4];

        let result = find_first_duplicated_frequency(&numbers);

        assert_eq!(10, result);
    }

    #[test]
    fn test2() {
        let numbers = vec![1, -1];

        let result = find_first_duplicated_frequency(&numbers);

        assert_eq!(0, result);
    }

    #[test]
    fn test3() {
        let numbers = vec![-6, 3, 8, 5, -6];

        let result = find_first_duplicated_frequency(&numbers);

        assert_eq!(5, result);
    }

    #[test]
    fn test4() {
        let numbers = vec![7, 7, -2, -7, -4];

        let result = find_first_duplicated_frequency(&numbers);

        assert_eq!(14, result);
    }
}
