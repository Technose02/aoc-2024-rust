pub fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {

    use super::part1;

    const PART1_TEST_INPUT: &str = r#"MMMSXXMASM
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
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }
}
