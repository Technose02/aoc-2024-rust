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

#[cfg(test)]
mod tests {

    use super::part1;

    const PART1_TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    const PART1_OUTPUT: usize = 11;

    #[test]
    fn day01_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }
}
