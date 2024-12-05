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
