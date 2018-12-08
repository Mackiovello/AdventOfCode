use super::get_input_vec;
use std::collections::{HashMap, HashSet};

pub fn problem_three_part_one() -> usize {
    let input = get_input_vec("three.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect();
    find_number_of_overlaps(input_refs)
}

pub fn problem_three_part_two() -> u32 {
    let input = get_input_vec("three.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect();
    let claims = parse_claims(input_refs);
    find_non_overlapping_claim(claims)
}

#[derive(Debug)]
struct Claim {
    id: u32,
    squares: Vec<(u32, u32)>,
}

fn parse_claims(claims: Vec<&str>) -> Vec<Claim> {
    claims
        .iter()
        .map(|c| Claim {
            id: c.split_whitespace().take(1).collect::<Vec<&str>>()[0]
                .trim_start_matches('#')
                .parse::<u32>()
                .unwrap(),
            squares: get_squares_from_claim(c),
        })
        .collect()
}

fn find_non_overlapping_claim(claims: Vec<Claim>) -> u32 {
    let mut values_to_id: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    claims.iter().for_each(|claim| {
        claim.squares.iter().for_each(|square| {
            let ids = values_to_id
                .entry(*square)
                .or_insert_with(|| vec![claim.id]);
            if !ids.contains(&claim.id) {
                ids.push(claim.id);
            }
        });
    });

    claims
        .iter()
        .filter(|v| {
            v.squares
                .iter()
                .all(|square| values_to_id[square].len() == 1)
        })
        .collect::<Vec<&Claim>>()[0]
        .id
}

fn get_squares_from_claim(claim: &str) -> Vec<(u32, u32)> {
    let claim_components = claim.split_whitespace().collect::<Vec<&str>>();

    let top_left_corner = claim_components[2]
        .trim_end_matches(':')
        .split(',')
        .map(|v| v.parse::<u32>().unwrap() + 1)
        .collect::<Vec<u32>>();

    let size = claim_components[3]
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
