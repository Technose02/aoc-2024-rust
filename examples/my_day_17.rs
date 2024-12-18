use aoc2024_rust::{day17, util::load_input};
use std::time::Instant;

// part 2: higher than 4235660

fn main() {
    let init_val = 105759271708800;

    let input = &load_input("input_day_17.txt");
    let mut start = Instant::now();
    println!("result of part1 for my input: {}", day17::part1(input));
    let dur_1 = start.elapsed();
    start = Instant::now();
    println!(
        "result of part2 for my input: {}",
        day17::part2(input, init_val, 1) // Range for outputs of len 16: 35184372088832 - 281474976710656
    );
    let dur_2 = start.elapsed();

    println!(
        "measured durations:\n\tpart1: {}ms\n\tpart2: {}ms",
        dur_1.as_millis(),
        dur_2.as_millis()
    );
}
