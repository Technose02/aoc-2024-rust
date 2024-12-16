use aoc2024_rust::{day16, util::load_input};
use std::time::Instant;

// 105512 is too high

fn main() {
    let input = &load_input("input_day_16.txt");
    let mut start = Instant::now();
    println!("result of part1 for my input: {}", day16::part1(input));
    let dur_1 = start.elapsed();
    start = Instant::now();
    println!("result of part2 for my input: {}", day16::part2(input));
    let dur_2 = start.elapsed();

    println!(
        "measured durations:\n\tpart1: {}ms\n\tpart2: {}ms",
        dur_1.as_millis(),
        dur_2.as_millis()
    );
}