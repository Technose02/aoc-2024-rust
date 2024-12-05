pub fn part1(input: &str) -> usize {
    let mut s = 0;
    let mut offset = 0;

    loop {
        let (res, mult_present, offs) = get_first_mult_and_cursor(offset, input);
        if !mult_present {
            break;
        }
        if let Some(res) = res {
            s += res;
            //        } else {
            //            break;
        }
        offset = offs;
    }
    s
}

fn get_first_mult_and_cursor(offset: usize, input: &str) -> (Option<usize>, bool, usize) {
    let mut chars = input.chars();
    if offset > 0 {
        _ = chars.nth(offset - 1);
    }

    let k = chars.as_str().find("mul(");
    if k.is_none() {
        return (None, false, offset);
    }
    let k = k.unwrap();

    let mut offset = offset + k + 4;
    chars.nth(k + 3);

    let mut num_buffer = ['0'; 100];
    let mut num_buffer_pos = 0;
    for d in chars.by_ref() {
        if d == ',' {
            offset += 1;
            break;
        }
        if !d.is_numeric() {
            return (None, true, offset);
        }
        num_buffer[num_buffer_pos] = d;
        num_buffer_pos += 1;
        offset += 1;
    }
    let num1: usize = num_buffer[..num_buffer_pos]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();

    num_buffer_pos = 0;
    for d in chars.by_ref() {
        if d == ')' {
            offset += 1;
            break;
        }
        if !d.is_numeric() {
            return (None, true, offset);
        }
        num_buffer[num_buffer_pos] = d;
        num_buffer_pos += 1;
        offset += 1;
    }
    let num2: usize = num_buffer[..num_buffer_pos]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();

    (Some(num1 * num2), true, offset)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_first_mult_and_cursor_works() {
        assert_eq!(
            get_first_mult_and_cursor(
                0,
                r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            ),
            (Some(8), true, 9)
        );
    }
}
