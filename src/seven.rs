use super::get_input_vec;
use regex::Regex;

// ACHOQRXSEKUGMYIWDZLNBFTJVP - correct
pub fn problem_seven_part_one() -> String {
    let input = get_input_vec("seven.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    find_order(&input_refs)
}

fn find_order(steps: &[&str]) -> String {
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
            .min_by(|a, b| {
                (a.0.chars().nth(0).unwrap() as i32).cmp(&(&(b.0.chars().nth(0).unwrap() as i32)))
            })
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

    order_removed.join("")
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
    fn sample() {
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
        let order = find_order(&input);
        println!("{:?}", order);

        // Then
        assert!(false);
    }
}
