use super::{parse_input, process_visits, Orientation};

pub fn part2(input: &str) -> usize {
    type PositionWithOrientation = ((usize, usize), Orientation);

    let mut visited_with_orientation = Vec::<PositionWithOrientation>::new();

    process_visits(parse_input(input), |pos, orientation| {
        visited_with_orientation.push((pos, orientation));
    });

    println!("{visited_with_orientation:?}");
    visited_with_orientation.len()
}
