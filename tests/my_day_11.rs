use aoc2024_rust::day11;
use aoc2024_rust::util::load_data;

#[test]
#[ignore = "day11 p1 not ready yet"]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_11.txt", "output_day_11.txt");
    assert_eq!(day11::part1(input), output_1.parse::<usize>().unwrap());
}

#[test]
#[ignore = "day11 p2 not ready yet"]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_11.txt", "output_day_11.txt");
    assert_eq!(day11::part2(input), output_2.parse::<usize>().unwrap());
}
