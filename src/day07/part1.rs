use super::Equation;

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
