use super::record_is_safe;

pub fn part1(input: &str) -> usize {
    let mut s = 0;
    for record in input.lines() {
        let slc = record
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if record_is_safe(slc) {
            s += 1;
        }
    }
    s
}

#[cfg(test)]
mod tests {

    use super::part1;

    const PART1_TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    const PART1_OUTPUT: usize = 2;

    #[test]
    fn day02_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }
}
