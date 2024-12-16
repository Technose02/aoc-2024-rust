mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const PART1_TEST_INPUT_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const PART1_OUTPUT_1: usize = 7036;

    const PART1_TEST_INPUT_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    const PART1_OUTPUT_2: usize = 11048;

    #[test]
    fn day16_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT_1), PART1_OUTPUT_1);
        assert_eq!(part1(PART1_TEST_INPUT_2), PART1_OUTPUT_2);
    }

    const PART2_TEST_INPUT: &str = r#""#;

    const PART2_OUTPUT: usize = 0;

    #[test]
    #[ignore = "day16 part2 not implemented yet"]
    fn day16_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
