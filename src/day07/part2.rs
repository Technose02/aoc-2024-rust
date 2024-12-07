use super::Equation;

fn concat(head: usize, tail: usize) -> usize {
    format!("{head}{tail}").parse().unwrap()
}

fn unconcat(concat_res: usize, tail: usize) -> Option<usize> {
    let s_res = concat_res.to_string();
    let s_tail = tail.to_string();

    let l_res = s_res.len();
    let l_tail = s_tail.len();

    if l_res > l_tail && s_res[l_res - l_tail..] == s_tail {
        Some(s_res[..l_res - l_tail].parse().unwrap())
    } else {
        None
    }
}

fn check_equation(equation: &Equation) -> bool {
    let Equation { test_value, args } = equation;
    let l = args.len();
    if l == 2 {
        let a = args[0];
        let b = args[1];
        return *test_value == a + b || *test_value == a * b || *test_value == concat(a, b);
    }
    let last = *args.last().unwrap();
    let unconcat_res = unconcat(*test_value, last);
    match (
        *test_value % last == 0,
        *test_value > last,
        unconcat_res.is_some(),
    ) {
        (true, true, true) => {
            check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(unconcat_res.unwrap(), &args[..l - 1]))
        }
        (true, true, false) => {
            check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1]))
        }
        (true, false, true) => {
            check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(unconcat_res.unwrap(), &args[..l - 1]))
        }
        (false, true, true) => {
            check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1]))
                || check_equation(&Equation::from_parts(unconcat_res.unwrap(), &args[..l - 1]))
        }
        (false, false, true) => {
            check_equation(&Equation::from_parts(unconcat_res.unwrap(), &args[..l - 1]))
        }
        (false, true, false) => {
            check_equation(&Equation::from_parts(*test_value - last, &args[..l - 1]))
        }
        (true, false, false) => {
            check_equation(&Equation::from_parts(*test_value / last, &args[..l - 1]))
        }
        (false, false, false) => false,
    }
}

pub fn part2(input: &str) -> usize {
    let mut res = 0;
    for line in input.lines() {
        let e = Equation::parse(line);
        if check_equation(&e) {
            res += e.test_value;
        }
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn concat_works() {
        assert_eq!(45867, concat(45, 867));
    }

    #[test]
    fn unconcat_works() {
        assert_eq!(Some(3456), unconcat(3456789, 789));
        assert_eq!(None, unconcat(3456789, 788));
        assert_eq!(None, unconcat(56, 788));
    }
}
