mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const PART1_TEST_INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const PART1_OUTPUT: usize = 1930;

    #[test]
    fn day12_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_TEST_INPUT: &str = r#""#;

    const PART2_OUTPUT: usize = 0;

    #[test]
    #[ignore = "day 12 part 2 not ready yet"]
    fn day12_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
