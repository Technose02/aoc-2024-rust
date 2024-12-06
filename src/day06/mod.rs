use std::collections::HashMap;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[derive(Clone)]
struct InputStats {
    pos: (usize, usize),
    orientation: Orientation,
    obstacles_by_row: HashMap<usize, Vec<usize>>,
    obstacles_by_col: HashMap<usize, Vec<usize>>,
    map_size: (usize, usize),
}

impl InputStats {
    pub fn add_obstacle(&mut self, at: (usize, usize)) {
        let (row, col) = at;
        if let Some(v) = self.obstacles_by_col.get_mut(&col) {
            v.push(row);
        } else {
            self.obstacles_by_col.insert(col, vec![row]);
        }

        if let Some(v) = self.obstacles_by_row.get_mut(&row) {
            v.push(col);
        } else {
            self.obstacles_by_row.insert(row, vec![col]);
        }
    }
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

fn process_visits(
    input_stats: InputStats,
    mut on_visit: impl FnMut((usize, usize), Orientation) -> bool,
) {
    let InputStats {
        mut pos,
        mut orientation,
        obstacles_by_row,
        obstacles_by_col,
        map_size: (row_count, col_count),
    } = input_stats;

    if !on_visit(pos, orientation) {
        return;
    }

    loop {
        match orientation {
            Orientation::Up => {
                let mut obstacles = {
                    if let Some(v) = obstacles_by_col.get(&pos.1) {
                        v.iter()
                            .copied()
                            .filter(|j| *j < pos.0)
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    }
                };
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
                let mut obstacles = {
                    if let Some(v) = obstacles_by_row.get(&pos.0) {
                        v.iter()
                            .copied()
                            .filter(|j| *j > pos.1)
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    }
                };
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
                let mut obstacles = {
                    if let Some(v) = obstacles_by_col.get(&pos.1) {
                        v.iter()
                            .copied()
                            .filter(|j| *j > pos.0)
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    }
                };
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
                let mut obstacles = {
                    if let Some(v) = obstacles_by_row.get(&pos.0) {
                        v.iter()
                            .copied()
                            .filter(|j| *j < pos.1)
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    }
                };
                if obstacles.is_empty() {
                    for v in (0..pos.1).rev() {
                        if !on_visit((pos.0, v), Orientation::Left) {
                            return;
                        };
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| b.cmp(&a));
                for v in (obstacles[0] + 1..pos.1).rev() {
                    if !on_visit((pos.0, v), Orientation::Left) {
                        return;
                    }
                }
                pos = (pos.0, obstacles[0] + 1);
                orientation = Orientation::Up;
                if !on_visit((pos.0, obstacles[0] + 1), Orientation::Up) {
                    return;
                }
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

    const PART2_OUTPUT: usize = 6;

    #[test]
    fn day06_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
