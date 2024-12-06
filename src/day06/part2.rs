use super::{parse_input, process_visits, Orientation};
use std::collections::HashSet;

pub fn part2(input: &str) -> usize {
    type PositionWithOrientation = ((usize, usize), Orientation);

    let mut visited_with_orientation = HashSet::<PositionWithOrientation>::new();

    process_visits(parse_input(input), |pos, orientation| {
        visited_with_orientation.insert((pos, orientation));
    });

    /* // print map für debugging (only for small samples!)
    let mut obstacles = HashSet::<(usize, usize)>::new();
    for (j, v) in obstacles_by_row {
        for i in v {
            obstacles.insert((j, i));
        }
    }

    let mut map = String::new();
    for j in 0..row_count {
        for i in 0..col_count {
            if visited.contains(&(j, i)) {
                map.push_str("X");
            } else {
                map.push_str(".");
            }
            if obstacles.contains(&(j, i)) {
                map.pop();
                map.push_str("#")
            }
        }
        map.push_str("\n");
    }
    println!("{map}");
    */

    println!("{visited_with_orientation:?}");
    visited_with_orientation.len()
}
