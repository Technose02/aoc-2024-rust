use aoc2024_rust::day14;
use aoc2024_rust::util::load_data;

#[test]
#[ignore = "day14 part1 not implemented yet"]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_14.txt", "output_day_14.txt");
    assert_eq!(day14::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day14 part2 not implemented yet"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_14.txt", "output_day_14.txt");
    assert_eq!(day14::part2(input), output_2.parse::<usize>().unwrap());
}
