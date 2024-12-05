mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    input
        .lines()
        .map(|l| l.trim().split("   "))
        .for_each(|mut s| {
            list_a.push(s.next().unwrap().parse().unwrap());
            list_b.push(s.next().unwrap().parse().unwrap());
        });

    (list_a, list_b)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    const PART1_OUTPUT: usize = 11;

    #[test]
    fn day01_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 31;

    #[test]
    fn day01_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
