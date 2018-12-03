use super::get_input_vec;
use std::collections::HashSet;

pub fn problem_three_part_one() -> usize {
    let input = get_input_vec("three.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect();
    find_number_of_overlaps(input_refs)
}

fn get_squares_from_claim(claim: &str) -> Vec<(u32, u32)> {
    let claim_components = claim.split_whitespace().skip(2).collect::<Vec<&str>>();

    let top_left_corner = claim_components[0]
        .trim_end_matches(':')
        .split(',')
        .map(|v| v.parse::<u32>().unwrap() + 1)
        .collect::<Vec<u32>>();

    let size = claim_components[1]
        .split('x')
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let top_row = (top_left_corner[0]..(top_left_corner[0] + size[0]))
        .map(|v| (v, top_left_corner[1]))
        .collect::<Vec<(u32, u32)>>();

    (top_left_corner[1]..(top_left_corner[1] + size[1]))
        .map(|v| top_row.iter().map(move |c| (c.0, v)))
        .flatten()
        .collect::<Vec<(u32, u32)>>()
}

fn find_number_of_overlaps(claims: Vec<&str>) -> usize {
    let mut overlaps = HashSet::new();
    let mut unique_elements = HashSet::new();
    claims
        .iter()
        .map(|v| get_squares_from_claim(v))
        .flatten()
        .for_each(|v| {
            if !unique_elements.insert(v) {
                overlaps.insert(v);
            }
        });

    overlaps.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_squares_from_claim() {
        // given
        let claim = "#1 @ 1,3: 2x2";
        let expected_squares = vec![(2, 4), (3, 4), (2, 5), (3, 5)];

        // that
        let actual_squares = get_squares_from_claim(claim);

        // then
        assert_eq!(actual_squares, expected_squares);
    }

    #[test]
    fn finds_number_of_overlaps() {
        // given
        let claims = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];

        // that
        let number_of_overlaps = find_number_of_overlaps(claims);

        // then
        assert_eq!(4, number_of_overlaps);
    }
}
