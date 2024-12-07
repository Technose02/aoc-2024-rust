use aoc2024_rust::{day06, util::load_input};
use std::time::Instant;

fn main() {
    let input = &load_input("input_day_06.txt");
    let start = Instant::now();
    println!("result of part1 for my input: {}", day06::part1(input));
    let dur_1 = start.elapsed();
    println!("result of part2 for my input: {}", day06::part2(input));
    let dur_2 = start.elapsed();

    println!(
        "measured durations:\n\tpart1: {}ms\n\tpart2 : {}ms",
        dur_1.as_millis(),
        dur_2.as_millis()
    );
}
