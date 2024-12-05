use std::collections::HashMap;

use super::parse_input;

pub fn part2(input: &str) -> usize {
    let (list_a, list_b) = parse_input(input);

    let mut histogram_a = HashMap::new();
    for n in list_a {
        if let Some(cnt) = histogram_a.get_mut(&n) {
            *cnt += 1;
        } else {
            histogram_a.insert(n, 1);
        }
    }

    let mut s = 0;
    for k in list_b {
        if let Some(cnt) = histogram_a.get(&k) {
            s += cnt * k;
        }
    }
    s
}
