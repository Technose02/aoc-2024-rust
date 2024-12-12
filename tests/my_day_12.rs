use aoc2024_rust::day12;
use aoc2024_rust::util::load_data;

#[test]
#[ignore = "day 12 part 1 not ready yet"]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_12.txt", "output_day_12.txt");
    assert_eq!(day12::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day 12 part 2 not ready yet"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_12.txt", "output_day_12.txt");
    assert_eq!(day12::part2(input), output_2.parse::<usize>().unwrap());
}
