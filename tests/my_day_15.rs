use aoc2024_rust::day15;
use aoc2024_rust::util::load_data;

#[test]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_15.txt", "output_day_15.txt");
    assert_eq!(day15::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day15 part2 still broken"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_15.txt", "output_day_15.txt");
    assert_eq!(day15::part2(input), output_2.parse::<usize>().unwrap());
}
