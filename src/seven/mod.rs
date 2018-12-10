use self::part_one::find_order;
use self::step::Step;

use super::get_input_vec;

use regex::Regex;
use std::collections::HashSet;

mod part_one;
mod step;

// ACHOQRXSEKUGMYIWDZLNBFTJVP - correct
// ACOQRXSEHKUGMYIZLNBWDFTJVP
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
    let parsed_steps: Vec<(&str, &str)> = parse_input(steps);
    let char_steps = to_char_steps(&parsed_steps);
    let constructed_steps = construct_steps(&char_steps);
    execute_steps(constructed_steps, workers, base_time).unwrap()
}

fn construct_steps(dependencies: &[(char, char)]) -> Vec<Step> {
    let unique_letters = to_unique_letters(dependencies);

    let steps: Vec<Step> = unique_letters.iter().map(|&u| Step::new(u)).collect();

    for i in 0..steps.len() {
        let representation = steps[i].get_representation();

        let dependency_representation: Vec<char> = dependencies
            .iter()
            .filter(|d| d.0 == representation)
            .map(|d| d.1)
            .collect();

        // look into refactoring with the position function
        let mut positions = Vec::new();
        for (i, _) in steps.iter().enumerate() {
            if dependency_representation.contains(&steps[i].get_representation()) {
                positions.push(i);
            }
        }

        for dependency_index in positions {
            steps[i].add_dependency(dependency_index);
        }
    }

    steps
}

fn to_char_steps(steps: &[(&str, &str)]) -> Vec<(char, char)> {
    steps
        .iter()
        .map(|s| {
            (
                s.0.chars().collect::<Vec<_>>()[0],
                s.1.chars().collect::<Vec<_>>()[0],
            )
        })
        .collect()
}

fn to_unique_letters<'a>(steps: &[(char, char)]) -> HashSet<char> {
    steps.iter().map(|s| vec![s.0, s.1]).flatten().collect()
}

fn execute_steps(steps: Vec<Step>, workers: u32, base_time: u32) -> Result<u32, String> {
    if workers == 0 {
        return Err("Can't execute steps with no workers.".to_owned());
    }

    let mut time = 0;
    let mut order: Vec<char> = Vec::new();

    loop {
        let next: Option<&Step> = steps
            .iter()
            .filter(|s| !s.is_finished())
            .filter(|s| s.can_be_finished(steps.iter().clone().map(|s| s).collect()))
            .max_by(|a, b| {
                (a.get_representation() as i32).cmp(&(&(b.get_representation() as i32)))
            });

        if next.is_none() {
            order.reverse();
            println!("{:?}", order.iter().collect::<String>());
            return Ok(time);
        }

        let step = next.unwrap();
        order.push(step.get_representation());
        step.finish();

        time += step.get_time_to_finish(base_time);
    }
}

fn parse_input<'a>(steps: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let reg =
        Regex::new(r"^Step (?P<first>\w) must be finished before step (?P<second>\w) can begin.$")
            .unwrap();

    steps
        .iter()
        .map(|s| {
            let captured = reg.captures(s).unwrap();
            (
                captured.name("first").unwrap(),
                captured.name("second").unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_workers_errs() {
        // Given
        let workers = 0;
        let base_time = 60;
        let steps = (68u8..72).map(|n| Step::new(n as char)).collect();

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(
            Err("Can't execute steps with no workers.".to_owned()),
            result
        );
    }

    #[test]
    fn no_steps_return_0() {
        // Given
        let workers = 3;
        let base_time = 60;
        let steps = vec![];

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(0), result);
    }

    #[test]
    fn one_step_returns_time_to_finish_step() {
        // Given
        let workers = 3;
        let base_time = 60;
        let steps = vec![Step::new('A')];
        let expected_to_finish = steps[0].get_time_to_finish(base_time);

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(expected_to_finish), result);
    }

    #[test]
    fn two_steps_without_dependencies_return_time_for_longest_with_two_workers() {
        // Given
        let workers = 2;
        let base_time = 60;
        let steps = vec![Step::new('A'), Step::new('B')];
        let expected_to_finish = steps[1].get_time_to_finish(base_time);

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(expected_to_finish), result);
    }

    #[test]
    fn two_steps_dependent_on_eachother_returns_combined_time() {
        // Given
        let workers = 2;
        let base_time = 60;
        let input = vec![('A', 'B')];

        // When
        let steps = construct_steps(&input);
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(123), result);
    }

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
}
