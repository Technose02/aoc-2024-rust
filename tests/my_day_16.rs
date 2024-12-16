#![allow(unused_variables)]

use aoc2024_rust::day16;
use aoc2024_rust::util::load_data;

#[test]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_16.txt", "output_day_16.txt");
    assert_eq!(day16::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day16 part2 not implemented yet"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_16.txt", "output_day_16.txt");
    assert_eq!(day16::part2(input), output_2.parse::<usize>().unwrap());
}
