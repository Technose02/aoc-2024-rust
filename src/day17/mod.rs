mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT_1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const TEST_INPUT_2: &str = r#"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    const PART1_OUTPUT_1: &str = "4,6,3,5,6,3,5,2,1,0";
    const PART1_OUTPUT_2: &str = "0,3,5,4,3,0";

    #[test]
    fn day17_part1_works() {
        assert_eq!(part1(TEST_INPUT_1), String::from(PART1_OUTPUT_1));
        assert_eq!(part1(TEST_INPUT_2), String::from(PART1_OUTPUT_2));
    }

    const PART2_OUTPUT: u64 = 117440;

    #[test]
    fn day17_part2_works() {
        assert_eq!(part2(TEST_INPUT_2), PART2_OUTPUT);
    }
}
