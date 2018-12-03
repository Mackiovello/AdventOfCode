use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod one;
mod two;

pub fn print_solutions() {
    println!(
        "Solution for problem 1, part 1: {}",
        one::problem_one_part_one()
    );
    println!(
        "Solution for problem 1, part 2: {}",
        one::problem_one_part_two()
    );
    println!(
        "Solution for problem 2, part 1: {}",
        two::problem_two_part_one()
    );
    println!(
        "Solution for problem 2, part 2: {}",
        two::problem_two_part_two()
    );
}

pub fn get_input_vec(file: &str) -> Vec<String> {
    // This is some dumpster fire error handling
    let file_name = format!("input/{}", file);
    let path = Path::new(&file_name);
    let mut file = File::open(&path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content.split_whitespace().map(|s| s.to_owned()).collect()
}

pub fn get_int_input_vec(file: &str) -> Vec<i32> {
    get_input_vec(file)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::print_solutions;

    #[test]
    fn solutions_execute() {
        print_solutions();
    }
}
