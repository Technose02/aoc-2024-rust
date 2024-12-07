mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const PART1_TEST_INPUT: &str = r#""#;

    const PART1_OUTPUT: usize = 0;

    #[test]
    fn day08_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_TEST_INPUT: &str = r#""#;

    const PART2_OUTPUT: usize = 0;

    #[test]
    fn day08_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
