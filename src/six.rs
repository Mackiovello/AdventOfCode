use super::get_input_vec;
use std::collections::HashMap;

// 5477 - too high
// 5358 - correct
pub fn problem_six_part_one() -> usize {
    let input = get_input_vec("six.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let points = parse_input(&input_refs);
    calculate_largest_area(&points)
}

fn parse_input(input: &[&str]) -> Vec<(i32, i32)> {
    input
        .iter()
        .map(|s| {
            let split = s
                .split(", ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (split[0], split[1])
        })
        .collect()
}

type ProximityHashMap = HashMap<(i32, i32), Vec<(i32, i32)>>;

fn calculate_largest_area(points: &[(i32, i32)]) -> usize {
    let max_x = points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_x = points.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_y = points.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let coordinates = (min_x..=max_x)
        .map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .flatten()
        .collect::<Vec<_>>();

    let mut nearest_points = find_nearest_points(&coordinates, &points);

    let mut perimeter = (min_x..=max_x)
        .map(|x| vec![(x, max_y + 1), (x, min_y - 1)])
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    perimeter.extend(
        (min_y..=max_y)
            .map(|y| vec![(max_x + 1, y), (min_x - 1, y)])
            .flatten(),
    );

    let nearest_perimeter_points = find_nearest_points(&perimeter, &points);

    nearest_perimeter_points.keys().for_each(|p| {
        nearest_points.remove(p);
    });

    nearest_points.values().map(|v| v.len()).max().unwrap()
}

fn find_nearest_points(coordinates: &[(i32, i32)], points: &[(i32, i32)]) -> ProximityHashMap {
    let mut nearest_points: ProximityHashMap = HashMap::new();

    coordinates.iter().for_each(|c| {
        let points_with_distance = points.iter().map(|p| (p, calculate_distance(c, p)));
        let closest_distance = points_with_distance
            .clone()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        let mut closest_points = points_with_distance
            .filter(|p| p.1 == closest_distance)
            .map(|p| p.0);

        if closest_points.clone().count() == 1 {
            let closest_point = closest_points.nth(0).unwrap();
            let vec = nearest_points.entry(*closest_point).or_default();
            vec.push(*c);
        }
    });

    nearest_points
}

fn calculate_distance(first: &(i32, i32), second: &(i32, i32)) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn sample() {
    //     let input = vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];

    //     // Given
    //     let parsed_input = parse_input(&input);

    //     let largest_area = calculate_largest_area(parsed_input);
    //     println!("{}", largest_area);

    //     // When
    //     assert!(false);

    //     // Then
    // }
}
