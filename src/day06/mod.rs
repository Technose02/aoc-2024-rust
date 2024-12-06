use std::collections::HashMap;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

struct InputStats {
    pos: (usize, usize),
    orientation: Orientation,
    obstacles_by_row: HashMap<usize, Vec<usize>>,
    obstacles_by_col: HashMap<usize, Vec<usize>>,
    map_size: (usize, usize),
}

fn parse_input(input: &str) -> InputStats {
    let mut obstacles_by_row = HashMap::<usize, Vec<usize>>::new();
    let mut obstacles_by_col = HashMap::<usize, Vec<usize>>::new();
    let mut pos = (0_usize, 0_usize);

    let mut lines = input.lines();
    let col_count = lines.next().unwrap().len();
    let row_count = lines.count() + 1;

    for (j, line) in input.lines().enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(i, c)| match c {
                '#' => {
                    if let Some(v) = obstacles_by_col.get_mut(&i) {
                        v.push(j)
                    } else {
                        obstacles_by_col.insert(i, vec![j]);
                    }
                    if let Some(v) = obstacles_by_row.get_mut(&j) {
                        v.push(i)
                    } else {
                        obstacles_by_row.insert(j, vec![i]);
                    }
                }
                '^' => pos = (j, i),
                _ => panic!("unexpected char '{c}' encountered"),
            });
    }

    InputStats {
        pos,
        orientation: Orientation::Up,
        obstacles_by_row,
        obstacles_by_col,
        map_size: (row_count, col_count),
    }
}

#[derive(PartialEq, Hash, Debug, Clone, Copy)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Eq for Orientation {}

fn process_visits(input_stats: InputStats, mut on_visit: impl FnMut((usize, usize), Orientation)) {
    let InputStats {
        mut pos,
        mut orientation,
        obstacles_by_row,
        obstacles_by_col,
        map_size: (row_count, col_count),
    } = input_stats;

    on_visit(pos, orientation);

    loop {
        match orientation {
            Orientation::Up => {
                let mut obstacles = obstacles_by_col
                    .get(&pos.1)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|j| *j < pos.0)
                    .collect::<Vec<usize>>();
                if obstacles.is_empty() {
                    for v in (0..pos.0).rev() {
                        on_visit((v, pos.1), Orientation::Up);
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| b.cmp(&a));
                for v in (obstacles[0] + 1..pos.0).rev() {
                    on_visit((v, pos.1), Orientation::Up);
                }
                pos = (obstacles[0] + 1, pos.1);
                orientation = Orientation::Right;
                on_visit((obstacles[0] + 1, pos.1), Orientation::Right);
            }
            Orientation::Right => {
                let mut obstacles = obstacles_by_row
                    .get(&pos.0)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|j| *j > pos.1)
                    .collect::<Vec<usize>>();
                if obstacles.is_empty() {
                    for v in pos.1 + 1..col_count {
                        on_visit((pos.0, v), Orientation::Right);
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| a.cmp(&b));
                for v in pos.1 + 1..obstacles[0] {
                    on_visit((pos.0, v), Orientation::Right);
                }
                pos = (pos.0, obstacles[0] - 1);
                orientation = Orientation::Down;
                on_visit((pos.0, obstacles[0] - 1), Orientation::Down);
            }
            Orientation::Down => {
                let mut obstacles = obstacles_by_col
                    .get(&pos.1)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|j| *j > pos.0)
                    .collect::<Vec<usize>>();
                if obstacles.is_empty() {
                    for v in pos.0 + 1..row_count {
                        on_visit((v, pos.1), Orientation::Down);
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| a.cmp(&b));
                for v in pos.0 + 1..obstacles[0] {
                    on_visit((v, pos.1), Orientation::Down);
                }
                pos = (obstacles[0] - 1, pos.1);
                orientation = Orientation::Left;
                on_visit((obstacles[0] - 1, pos.1), Orientation::Left);
            }
            Orientation::Left => {
                let mut obstacles = obstacles_by_row
                    .get(&pos.0)
                    .unwrap()
                    .iter()
                    .copied()
                    .filter(|j| *j < pos.1)
                    .collect::<Vec<usize>>();
                if obstacles.is_empty() {
                    for v in (0..pos.1).rev() {
                        on_visit((pos.0, v), Orientation::Left);
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| b.cmp(&a));
                for v in (obstacles[0] + 1..pos.1).rev() {
                    on_visit((pos.0, v), Orientation::Left);
                }
                pos = (pos.0, obstacles[0] + 1);
                orientation = Orientation::Up;
                on_visit((pos.0, obstacles[0] + 1), Orientation::Up);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    const PART1_OUTPUT: usize = 41;

    #[test]
    fn day06_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 0;

    #[test]
    fn day06_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
