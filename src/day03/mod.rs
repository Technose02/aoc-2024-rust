mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const PART1_TEST_INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    const PART1_OUTPUT: usize = 161;

    #[test]
    fn day03_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_TEST_INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    const PART2_OUTPUT: usize = 48;

    #[test]
    fn day03_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
