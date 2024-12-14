use aoc2024_rust::day13;
use aoc2024_rust::util::load_data;

#[test]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_13.txt", "output_day_13.txt");
    assert_eq!(day13::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_13.txt", "output_day_13.txt");
    assert_eq!(day13::part2(input), output_2.parse::<usize>().unwrap());
}
