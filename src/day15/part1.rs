use std::collections::{HashMap, HashSet};

const EMPTY: char = '.';
const BOX: char = 'O';
const WALL: char = '#';
const ROBOT: char = '@';

pub enum MoveDir {
    Up,
    Right,
    Down,
    Left,
}

impl MoveDir {
    pub fn apply_to(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            MoveDir::Up => (pos.0 - 1, pos.1),
            MoveDir::Right => (pos.0, pos.1 + 1),
            MoveDir::Down => (pos.0 + 1, pos.1),
            MoveDir::Left => (pos.0, pos.1 - 1),
        }
    }
}

#[allow(dead_code)]
pub struct StorageState {
    height: usize,
    width: usize,
    boxes: HashMap<usize, (usize, usize)>,
    walls: HashSet<(usize, usize)>,
    robot_pos: (usize, usize),
}

impl StorageState {
    pub fn is_wall(&self, at: (usize, usize)) -> bool {
        if self.walls.iter().any(|p| *p == at) {
            return true;
        }
        false
    }
    pub fn get_box(&self, at: (usize, usize)) -> Option<usize> {
        self.boxes.iter().find(|&(_, pos)| *pos == at).map(|e| *e.0)
    }
    fn move_if_possible(&mut self, pos: (usize, usize), dir: &MoveDir) -> Option<(usize, usize)> {
        let dest = dir.apply_to(pos);

        if self.is_wall(dest) {
            return None;
        }

        if let Some(id) = self.get_box(dest) {
            if let Some(new_box_pos) = self.move_if_possible(dest, dir) {
                *self.boxes.get_mut(&id).unwrap() = new_box_pos;
                Some(dest)
            } else {
                None
            }
        } else {
            Some(dest)
        }
    }

    #[allow(unused)]
    pub fn run_robot_movements(&mut self, movements: Vec<MoveDir>) -> usize {
        for (k, dir) in movements.iter().enumerate() {
            if let Some(next_robot_pos) = self.move_if_possible(self.robot_pos, dir) {
                self.robot_pos = next_robot_pos;
            }
            //println!("\nafter {_k} moves:\n");
            //self.print();
        }

        self.calc_gps_sum()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for j in 0..self.height {
            let mut row = String::with_capacity(self.width);
            for i in 0..self.width {
                if self.is_wall((j, i)) {
                    row.push(WALL);
                } else if self.get_box((j, i)).is_some() {
                    row.push(BOX);
                } else if self.robot_pos == (j, i) {
                    row.push(ROBOT);
                } else {
                    row.push(EMPTY);
                }
            }
            println!("{row}");
        }
    }

    pub fn calc_gps_sum(&self) -> usize {
        let mut s = 0;
        for (j, i) in self.boxes.values() {
            s += j * 100 + i;
        }
        s
    }
}

pub fn parse_input(input: &str) -> (StorageState, Vec<MoveDir>) {
    let mut lines = input.lines();

    let mut walls = HashSet::new();
    let mut boxes = HashMap::new();
    let mut robot_pos = None;

    let mut width = None;
    let mut next_box_id = 0;

    let mut j = 0;
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut i = 0;
        for c in line.chars() {
            match c {
                BOX => {
                    boxes.insert(next_box_id, (j, i));
                    next_box_id += 1;
                }
                WALL => {
                    walls.insert((j, i));
                }
                ROBOT => {
                    robot_pos = Some((j, i));
                }
                EMPTY => (),
                _ => panic!(
                    "unexpected character '{c}' encountered while parsing initial StorageState"
                ),
            }
            i += 1;
        }
        if width.is_none() {
            width = Some(i);
        }
        j += 1;
    }

    let mut movements = Vec::new();
    lines.flat_map(|s| s.chars()).for_each(|c| match c {
        '^' => movements.push(MoveDir::Up),
        '>' => movements.push(MoveDir::Right),
        'v' => movements.push(MoveDir::Down),
        '<' => movements.push(MoveDir::Left),
        _ => panic!("unexpected character '{c}' encountered while parsing movements"),
    });

    (
        StorageState {
            height: j,
            width: width.unwrap(),
            walls,
            boxes,
            robot_pos: robot_pos.unwrap(),
        },
        movements,
    )
}

pub fn part1(input: &str) -> usize {
    let (mut storage_state, movements) = parse_input(input);

    storage_state.run_robot_movements(movements)
}
