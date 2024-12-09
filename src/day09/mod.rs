mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"2333133121414131402"#;

    const PART1_OUTPUT: usize = 1928;

    #[test]
    fn day09_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 0;

    #[test]
    #[ignore = "day09 part1 not yet implemented"]
    fn day09_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
