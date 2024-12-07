mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[derive(Debug)]
struct Equation {
    test_value: usize,
    args: Vec<usize>,
}

impl Equation {
    pub fn parse(line: &str) -> Self {
        let mut s = line.split(":");
        let test_value: usize = s.next().unwrap().trim().parse().unwrap();
        let args = s
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        Equation { test_value, args }
    }

    pub fn from_parts(test_value: usize, args: &[usize]) -> Self {
        Equation {
            test_value,
            args: args.to_vec(),
        }
    }
}

fn check_equation(equation: &Equation) -> bool {
    let Equation { test_value, args } = equation;
    let l = args.len();
    if l == 2 {
        let a = args[0];
        let b = args[1];
        return *test_value == a + b || *test_value == a * b;
    }
    let last = *args.last().unwrap();
    match (*test_value % last == 0, *test_value > last) {
        (true, true) => {
            check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1]))
        }
        (true, false) => check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1])),
        (false, true) => check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1])),
        (false, false) => false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const PART1_TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    const PART1_OUTPUT: usize = 3749;

    #[test]
    fn day07_part1_works() {
        assert_eq!(part1(PART1_TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    const PART2_OUTPUT: usize = 0;

    #[test]
    fn day07_part2_works() {
        assert_eq!(part2(PART2_TEST_INPUT), PART2_OUTPUT);
    }
}
