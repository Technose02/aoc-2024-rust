pub fn part2(input: &str) -> usize {
    let mut s = 0;
    let mut offset = 0;

    let mut enabled = true;
    loop {
        let (res, mult_present, offs, checkenabled) =
            get_first_mult_and_cursor(enabled, offset, input);
        enabled = checkenabled;
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

fn checkenabled(enabled_at_offset: bool, offset: usize, mul_pos: usize, input: &str) -> bool {
    let mut enabled = enabled_at_offset;
    let mut start = offset;
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt >= 100 {
            break;
        }
        if start > mul_pos {
            break;
        }
        let portion = &input[start..mul_pos].to_string();
        if let Some(i) = portion.find("do()") {
            enabled = true;
            start += i + 1;
        } else {
            break;
        }
    }
    loop {
        cnt += 1;
        if cnt >= 100 {
            break;
        }
        if start > mul_pos {
            break;
        }
        let portion = &input[start..mul_pos].to_string();
        if let Some(i) = portion.find("don't()") {
            enabled = false;
            start += i + 1;
        } else {
            break;
        }
    }

    enabled
}

fn get_first_mult_and_cursor(
    enabled_at_offset: bool,
    offset: usize,
    input: &str,
) -> (Option<usize>, bool, usize, bool) {
    let mut chars = input.chars();
    if offset > 0 {
        _ = chars.nth(offset - 1);
    }

    let k = chars.as_str().find("mul(");
    if k.is_none() {
        return (None, false, offset, enabled_at_offset);
    }
    let k = k.unwrap();

    if !checkenabled(enabled_at_offset, offset, offset + k, input) {
        return (None, true, offset + k + 4, false);
    }

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
            return (None, true, offset, true);
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
            return (None, true, offset, true);
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

    (Some(num1 * num2), true, offset, true)
}
