#![allow(unused_variables)]

use aoc2024_rust::day10;
use aoc2024_rust::util::load_data;

#[test]
#[ignore = "day10 not yet implemented"]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_10.txt", "output_day_10.txt");
    assert_eq!(day10::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day10 not yet implemented"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_10.txt", "output_day_10.txt");
    assert_eq!(day10::part2(input), output_2.parse::<usize>().unwrap());
}
