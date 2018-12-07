use super::get_input_vec;

// 10638 - correct on first
pub fn problem_five_part_one() -> usize {
    let input = get_input_vec("five.txt");
    let input_str = input.first().unwrap();
    reduced_size(input_str)
}

// 4944 - correct on first
pub fn problem_five_part_two() -> usize {
    let input = get_input_vec("five.txt");
    let input_str = input.first().unwrap();

    let all_letters_ascii = (65u8..91).zip(97u8..123);
    let all_letters = all_letters_ascii.map(|c| (c.0 as char, c.1 as char));

    all_letters
        .map(|polymer_pair| {
            let without_polymer = input_str
                .chars()
                .filter(|&c| c != polymer_pair.0)
                .filter(|&c| c != polymer_pair.1)
                .collect::<String>();
            reduced_size(&without_polymer)
        })
        .min()
        .unwrap()
}

fn reduced_size(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut result: Vec<Vec<char>> = vec![chars];
    loop {
        let last_result = result.last().unwrap();
        let reduced = reduce(last_result);
        if reduced.len() == last_result.len() {
            break;
        }
        result.push(reduced);
    }
    result.last().unwrap().len()
}

fn reduce(chars: &[char]) -> Vec<char> {
    chars.iter().fold(Vec::new(), |mut a, b| {
        let last_value = a.last();
        if last_value.is_some() && (*last_value.unwrap() as i32 - *b as i32).abs() == 32 {
            a.pop();
            a
        } else {
            a.push(*b);
            a
        }
    })
}
