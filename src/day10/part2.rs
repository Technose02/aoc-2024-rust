use super::Map;

pub fn part2(input: &str) -> usize {
    let map = Map::parse(input);
    let mut sum = 0;
    for head in map.potential_trails() {
        sum += map.check_subtrail_start(head).len();
    }

    sum
}
