use crate::day07::check_equation;

use super::Equation;

pub fn part1(input: &str) -> usize {
    let mut res = 0;
    for line in input.lines() {
        let e = Equation::parse(line);
        if check_equation(&e) {
            res += e.test_value;
        }
    }
    res
}
