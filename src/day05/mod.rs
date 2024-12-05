mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

type Rules = Vec<(u32, u32)>;

fn parse_input(input: &str) -> (Rules, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut lines = input.lines();

    // parse rules
    for l in &mut lines {
        if l.trim().is_empty() {
            break;
        }
        let mut nums = l.split('|');
        rules.push((
            nums.next().unwrap().parse::<u32>().unwrap(),
            nums.next().unwrap().parse::<u32>().unwrap(),
        ));
    }

    // parse updates
    for l in &mut lines {
        updates.push(
            l.split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    (rules, updates)
}

fn filter_relevant_rules(rules: Rules, update: &[u32]) -> Rules {
    rules
        .into_iter()
        .filter(|r| update.contains(&r.0) && update.contains(&r.1))
        .collect::<Rules>()
}

fn check_update(rules: Rules, update: &[u32]) -> bool {
    if update.is_empty() || rules.is_empty() {
        return true;
    }
    let l = update.len();

    let mut new_ruleset = Vec::new();

    let last_page = update[l - 1];
    for r in rules {
        if r.0 == last_page {
            return false;
        }
        if r.1 != last_page {
            new_ruleset.push(r);
        }
    }

    check_update(new_ruleset, &update[0..l - 1])
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    const PART1_OUTPUT: usize = 143;

    #[test]
    fn day05_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 123;

    #[test]
    fn day05_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }

    #[test]
    fn parse_input_works() {
        let (rules, updates) = parse_input(TEST_INPUT);
        assert_eq!(rules.len(), 21);
        assert_eq!(rules[0], (47, 53));
        assert_eq!(rules[1], (97, 13));
        assert_eq!(rules[2], (97, 61));
        assert_eq!(rules[3], (97, 47));
        assert_eq!(rules[4], (75, 29));
        assert_eq!(rules[5], (61, 13));
        assert_eq!(rules[6], (75, 53));
        assert_eq!(rules[7], (29, 13));
        assert_eq!(rules[8], (97, 29));
        assert_eq!(rules[9], (53, 29));
        assert_eq!(rules[10], (61, 53));
        assert_eq!(rules[11], (97, 53));
        assert_eq!(rules[12], (61, 29));
        assert_eq!(rules[13], (47, 13));
        assert_eq!(rules[14], (75, 47));
        assert_eq!(rules[15], (97, 75));
        assert_eq!(rules[16], (47, 61));
        assert_eq!(rules[17], (75, 61));
        assert_eq!(rules[18], (47, 29));
        assert_eq!(rules[19], (75, 13));
        assert_eq!(rules[20], (53, 13));

        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(updates[1], vec![97, 61, 53, 29, 13]);
        assert_eq!(updates[2], vec![75, 29, 13]);
        assert_eq!(updates[3], vec![75, 97, 47, 61, 53]);
        assert_eq!(updates[4], vec![61, 13, 29]);
        assert_eq!(updates[5], vec![97, 13, 75, 29, 47]);
    }
}
