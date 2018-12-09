#![feature(vec_remove_item)]

extern crate chrono;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod five;
mod four;
mod one;
mod seven;
mod six;
mod three;
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
    println!(
        "Solution for problem 3, part 1: {}",
        three::problem_three_part_one()
    );
    println!(
        "Solution for problem 3, part 2: {}",
        three::problem_three_part_two()
    );
    println!(
        "Solution for problem 4, part 1: {}",
        four::problem_four_part_one()
    );
    println!(
        "Solution for problem 4, part 2: {}",
        four::problem_four_part_two()
    );
    println!(
        "Solution for problem 5, part 1: {}",
        five::problem_five_part_one()
    );
    println!(
        "Solution for problem 5, part 2: {}",
        five::problem_five_part_two()
    );
    println!(
        "Solution for problem 6, part 1: {}",
        six::problem_six_part_one()
    );
    println!(
        "Solution for problem 6, part 2: {}",
        six::problem_six_part_two()
    );
    println!(
        "Solution for problem 7, part 1: {}",
        seven::problem_seven_part_one()
    );
    println!(
        "Solution for problem 7, part 2: {}",
        seven::problem_seven_part_two()
    );
}

pub fn get_input_vec(file: &str) -> Vec<String> {
    // This is some dumpster fire error handling
    let file_name = format!("input/{}", file);
    let path = Path::new(&file_name);
    let mut file = File::open(&path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content.split('\n').map(|s| s.to_owned()).collect()
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

    // #[test]
    // fn solutions_execute() {
    //     print_solutions();
    // }
}
