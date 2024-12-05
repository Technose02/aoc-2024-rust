use super::{check_update, filter_relevant_rules, parse_input, Rules};

fn determine_incorrect_updates(rules: Rules, updates: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut incorrect_updates = Vec::new();

    for u in updates {
        let relevant_rules = filter_relevant_rules(rules.clone(), u);
        if !check_update(relevant_rules, u) {
            incorrect_updates.push(u.clone());
        }
    }
    incorrect_updates
}

fn fix_update(mut update: Vec<u32>, rules: Rules, mut fixed_update: Vec<u32>) -> Vec<u32> {
    let rules = filter_relevant_rules(rules, &update);

    if rules.is_empty() || update.len() <= 1 {
        fixed_update.append(&mut update);
        return fixed_update;
    }

    let mut next_update = Vec::new();
    for p in update {
        if rules.iter().map(|r| r.1).filter(|k| *k == p).count() == 0 {
            fixed_update.push(p);
        } else {
            next_update.push(p);
        }
    }

    fix_update(next_update, rules, fixed_update)
}

pub fn part2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);

    let incorrect_updates = determine_incorrect_updates(rules.clone(), &updates);

    let mut s = 0;
    for update in incorrect_updates {
        let update = fix_update(update, rules.clone(), Vec::new());
        s += update[(update.len() - 1) / 2]
    }

    s as usize
}
