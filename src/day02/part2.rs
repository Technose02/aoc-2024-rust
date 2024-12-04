use super::{record_is_safe, skip_at_slice, SplitSlice};

pub fn part2(input: &str) -> usize {
    let mut s = 0;
    'outer: for record in input.lines() {
        let slc = record
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if record_is_safe(SplitSlice::from_slice(&slc)) {
            s += 1;
            continue 'outer;
        }
        for i in 0..slc.len() {
            if record_is_safe(skip_at_slice(&slc, i)) {
                s += 1;
                continue 'outer;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {

    use crate::day02::{skip_at_slice, SplitSlice};

    use super::part2;

    const PART2_TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    const PART2_OUTPUT: usize = 4;

    #[test]
    fn day02_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }

    #[test]
    fn iterating_split_splice_works() {
        for i in 0..5 {
            assert_eq!(
                vec![0, 1, 2, 3, 4],
                SplitSlice::from_split_at(&vec![0, 1, 2, 3, 4], i).collect::<Vec<i32>>(),
            );
        }
    }

    #[test]
    fn skip_at_slice_works() {
        assert_eq!(
            vec![1, 2, 3, 4],
            skip_at_slice(&vec![0, 1, 2, 3, 4], 0).collect::<Vec<i32>>(),
        );
        assert_eq!(
            vec![0, 2, 3, 4],
            skip_at_slice(&vec![0, 1, 2, 3, 4], 1).collect::<Vec<i32>>(),
        );
        assert_eq!(
            vec![0, 1, 3, 4],
            skip_at_slice(&vec![0, 1, 2, 3, 4], 2).collect::<Vec<i32>>(),
        );
        assert_eq!(
            vec![0, 1, 2, 4],
            skip_at_slice(&vec![0, 1, 2, 3, 4], 3).collect::<Vec<i32>>(),
        );
        assert_eq!(
            vec![0, 1, 2, 3],
            skip_at_slice(&vec![0, 1, 2, 3, 4], 4).collect::<Vec<i32>>(),
        );
    }
}
