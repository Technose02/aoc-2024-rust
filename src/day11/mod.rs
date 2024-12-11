mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"125 17"#;

    const PART1_OUTPUT: usize = 55312;

    #[test]
    fn day11_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 55312;

    #[test]
    #[ignore = "day11 p2 not working yet"]
    fn day11_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
