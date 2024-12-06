use std::collections::{HashMap, HashSet};

struct InputStats {
    pos: (usize, usize),
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
        obstacles_by_row,
        obstacles_by_col,
        map_size: (row_count, col_count),
    }
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: &str) -> usize {
    let InputStats {
        mut pos,
        obstacles_by_row,
        obstacles_by_col,
        map_size: (row_count, col_count),
    } = parse_input(input);

    let mut visited = HashSet::<(usize, usize)>::new();
    visited.insert(pos);
    let mut orientation = Orientation::Up;

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
                    for v in 0..pos.0 {
                        visited.insert((v, pos.1));
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| b.cmp(&a));
                for v in obstacles[0] + 1..pos.0 {
                    visited.insert((v, pos.1));
                }
                pos = (obstacles[0] + 1, pos.1);
                orientation = Orientation::Right;
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
                        visited.insert((pos.0, v));
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| a.cmp(&b));
                for v in pos.1 + 1..obstacles[0] {
                    visited.insert((pos.0, v));
                }
                pos = (pos.0, obstacles[0] - 1);
                orientation = Orientation::Down;
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
                        visited.insert((v, pos.1));
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| a.cmp(&b));
                for v in pos.0 + 1..obstacles[0] {
                    visited.insert((v, pos.1));
                }
                pos = (obstacles[0] - 1, pos.1);
                orientation = Orientation::Left;
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
                    for v in 0..pos.1 {
                        visited.insert((pos.0, v));
                    }
                    break;
                }
                obstacles.sort_by(|&a, &b| b.cmp(&a));
                for v in obstacles[0] + 1..pos.1 {
                    visited.insert((pos.0, v));
                }
                pos = (pos.0, obstacles[0] + 1);
                orientation = Orientation::Up;
            }
        }
    }

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

    visited.len()
}
