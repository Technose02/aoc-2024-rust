use super::parse_input;

pub fn part1(input: &str) -> usize {
    let (mut list_a, mut list_b) = parse_input(input);

    list_a.sort();
    list_b.sort();

    list_a
        .iter()
        .zip(list_b.iter())
        .map(|(&a, &b)| b.abs_diff(a))
        .sum()
}
