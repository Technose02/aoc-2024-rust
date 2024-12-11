use super::StoneCounts;

pub fn part1(input: &str) -> usize {
    let mut stone_counts = StoneCounts::new(
        input
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    );

    for _ in 0..25 {
        stone_counts.blink();
    }

    stone_counts.count()
}
