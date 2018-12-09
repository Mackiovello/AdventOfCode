use super::parse_input;

pub fn find_order(steps: &[&str]) -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

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
}
