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
