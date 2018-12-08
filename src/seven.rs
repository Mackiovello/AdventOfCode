use super::get_input_vec;
use regex::Regex;
use std::cmp::Ordering;

// ACHOQRXSEKUGMYIWDZLNBFTJVP - correct
pub fn problem_seven_part_one() -> String {
    let input = get_input_vec("seven.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    find_order(&input_refs).unwrap()
}

pub fn problem_seven_part_two() -> u32 {
    let input = get_input_vec("seven.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    time_to_complete(&input_refs, 5, 60)
}

fn time_to_complete(steps: &[&str], workers: u32, base_time: u32) -> u32 {
    0
}

fn find_order(steps: &[&str]) -> Result<String, String> {
    let parsed_steps = parse_input(steps);
    let mut order_removed: Vec<&str> = Vec::new();
    let mut removed_steps: Vec<(String, String)> = Vec::new();

    let mut all_letters: Vec<&str> = parsed_steps
        .iter()
        .map(|s| vec![&(*s.0), &(*s.1)])
        .flatten()
        .collect::<Vec<&str>>();

    all_letters.sort_unstable();
    all_letters.dedup();

    let all_unique_letters = all_letters.clone();

    let letter_length = all_unique_letters.len();

    while order_removed.len() != letter_length {
        let to_free = all_unique_letters
            .clone()
            .into_iter()
            .filter(|l| !order_removed.contains(l))
            .map(|letter| {
                (
                    letter,
                    parsed_steps
                        .iter()
                        .filter(|s| !removed_steps.contains(s))
                        .map(|step| &step.1)
                        .filter(move |&x| x == letter)
                        .count()
                        == 0,
                )
            })
            .filter(|v| v.1)
            .min_by(|a, b| ascii_code_ordering(a.0, b.0).unwrap())
            .unwrap()
            .0;

        let steps_to_remove = parsed_steps
            .iter()
            .filter(|s| s.0 == to_free || s.1 == to_free);

        for i in steps_to_remove {
            removed_steps.push(i.clone());
        }
        removed_steps.sort_unstable();
        removed_steps.dedup();

        order_removed.push(&to_free);
    }

    Ok(order_removed.join(""))
}

fn ascii_code_ordering(str1: &str, str2: &str) -> Result<std::cmp::Ordering, String> {
    let str1_chars = str1.chars().collect::<Vec<_>>();
    let str2_chars = str2.chars().collect::<Vec<_>>();

    if str1_chars.len() != 1 || str2_chars.len() != 1 {
        return Err("The input has to be a single ascii character long".to_owned());
    }

    let comparison = (str1_chars[0] as i32).cmp(&(&(str2_chars[0] as i32)));
    Ok(comparison)
}

fn time_for_step(step: &str) -> Result<i32, String> {
    let base_time = 60;

    let chars = step.chars().collect::<Vec<_>>();
    if chars.len() != 1 {
        return Err("The step has to be one letter.".to_owned());
    }

    // Save to unwrap since we've already
    // established that there's one character
    let character = chars.first().unwrap();

    let ascii_code = *character as i32;

    if ascii_code < 65 || ascii_code > 90 {
        return Err("The step has to be an uppercase letter.".to_owned());
    }

    // So A, which is 65, becomes 1, B becomes 2 etc.
    let normalized_ascii_code = ascii_code - 64;
    Ok(normalized_ascii_code + base_time)
}

fn parse_input(steps: &[&str]) -> Vec<(String, String)> {
    let reg =
        Regex::new(r"^Step (?P<first>\w) must be finished before step (?P<second>\w) can begin.$")
            .unwrap();

    steps
        .iter()
        .map(|s| {
            let captured = reg.captures(s).unwrap();
            (
                captured.name("first").unwrap().to_string(),
                captured.name("second").unwrap().to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_2() {
        // Given
        let input = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];
        // When
        let result = time_to_complete(&input, 2, 0);

        // Then
        assert_eq!(15, result);
    }

    #[test]
    fn ascii_code_ordering_err_on_non_one_char_strings() {
        let inputs = vec![("A", "AB"), ("JK", "U"), ("IP", "QE")];

        for input in inputs {
            let result = ascii_code_ordering(input.0, input.1);
            assert_eq!(
                Err("The input has to be a single ascii character long".to_owned()),
                result
            );
        }
    }

    #[test]
    fn ascii_code_ordering_works() {
        let inputs = vec![
            ("A", "B", Ok(Ordering::Less)),
            ("B", "A", Ok(Ordering::Greater)),
            ("B", "B", Ok(Ordering::Equal)),
        ];

        for input in inputs {
            let result = ascii_code_ordering(input.0, input.1);
            assert_eq!(input.2, result);
        }
    }

    #[test]
    fn time_for_steps_works() {
        let expected_results = vec![
            ("A", Ok(61)),
            ("B", Ok(62)),
            ("Z", Ok(86)),
            ("AB", Err("The step has to be one letter.".to_owned())),
            (
                "b",
                Err("The step has to be an uppercase letter.".to_owned()),
            ),
        ];

        for i in expected_results {
            let actual_result = time_for_step(i.0);
            assert_eq!(i.1, actual_result);
        }
    }

    #[test]
    fn sample() {
        assert_eq!(
            problem_seven_part_one(),
            "ACHOQRXSEKUGMYIWDZLNBFTJVP".to_owned()
        );
    }
}
