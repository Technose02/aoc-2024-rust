mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const PART1_OUTPUT: usize = 18;

    #[test]
    fn day_04_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 9;

    #[test]
    fn day_04_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
