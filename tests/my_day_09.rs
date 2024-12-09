#![allow(unused_variables)]

use aoc2024_rust::day09;
use aoc2024_rust::util::load_data;

#[test]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_09.txt", "output_day_09.txt");
    assert_eq!(day09::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day09 part1 not yet implemented"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_09.txt", "output_day_09.txt");
    assert_eq!(day09::part2(input), output_2.parse::<usize>().unwrap());
}
