use std::collections::HashMap;

use super::parse_input;

pub fn part2(input: &str) -> usize {
    let (list_a, mut list_b) = parse_input(input);

    list_b.sort();

    let mut sum = 0;
    let iter = list_a.iter();
    let mut b_offs = 0;
    let mut counts = HashMap::<usize, usize>::new();
    for &n in iter {
        let cnt = if let Some(cnt) = counts.get(&n) {
            *cnt
        } else {
            let mut cnt = 0_usize;
            let slice_of_b = list_b[b_offs..].iter();
            for &i in slice_of_b {
                if i > n {
                    break;
                }
                b_offs += 1;

                if i < n {
                    continue;
                }
                if i == n {
                    cnt += 1;
                }
            }
            counts.insert(n, cnt);
            cnt
        };
        sum += n * cnt;
    }

    sum
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
