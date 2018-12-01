mod one;

pub fn print_solutions() {
    println!(
        "Solution for problem 1, part 1: {}",
        one::problem_one_part_one()
    );
    println!(
        "Solution for problem 1, part 2: {}",
        one::problem_one_part_two()
    );
}

#[cfg(test)]
mod tests {
    use super::print_solutions;

    #[test]
    fn solutions_execute() {
        print_solutions();
    }
}
