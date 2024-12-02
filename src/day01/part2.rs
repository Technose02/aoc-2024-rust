use std::collections::HashMap;

use super::parse_input;

pub fn part2(input: &str) -> usize {
    let (list_a, list_b) = parse_input(input);

    let mut histogram_a = HashMap::new();
    for n in list_a {
        if let Some(cnt) = histogram_a.get_mut(&n) {
            *cnt += 1;
        } else {
            histogram_a.insert(n, 1);
        }
    }

    let mut s = 0;
    for k in list_b {
        if let Some(cnt) = histogram_a.get(&k) {
            s += cnt * k;
        }
    }
    s
}

#[cfg(test)]
mod tests {

    use super::part2;

    const PART2_TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    const PART2_OUTPUT: usize = 31;

    #[test]
    fn part2_should_work() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
