use aoc2024_rust::day14;
use aoc2024_rust::util::load_data;

#[test]
fn test_part_1() {
    let (input, output_1, _) = &load_data("input_day_14.txt", "output_day_14.txt");
    assert_eq!(
        day14::part1::<103, 101>(input),
        output_1.parse::<usize>().unwrap()
    );
}

#[test]
fn test_part_2() {
    let (input, _, output_2) = &load_data("input_day_14.txt", "output_day_14.txt");
    assert_eq!(
        day14::part2::<103, 101>(input),
        output_2.parse::<usize>().unwrap()
    );
}
