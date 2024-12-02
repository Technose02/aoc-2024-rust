mod part1;
mod part2;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    input
        .lines()
        .map(|l| l.trim().split("   "))
        .for_each(|mut s| {
            list_a.push(s.next().unwrap().parse().unwrap());
            list_b.push(s.next().unwrap().parse().unwrap());
        });

    (list_a, list_b)
}

pub use part1::part1;
pub use part2::part2;
