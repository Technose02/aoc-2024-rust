use aoc2024_rust::day08;
use aoc2024_rust::util::load_data;

#[test]
#[ignore]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_08.txt", "output_day_08.txt");
    assert_eq!(day08::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_08.txt", "output_day_08.txt");
    assert_eq!(day08::part2(input), output_2.parse::<usize>().unwrap());
}
