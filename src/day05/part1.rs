use super::{check_update, filter_relevant_rules, parse_input};

pub fn part1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let mut s = 0;
    for u in updates {
        let relevant_rules = filter_relevant_rules(rules.clone(), &u);
        if check_update(relevant_rules, &u) {
            s += u[(u.len() - 1) / 2]
        }
    }

    s as usize
}
