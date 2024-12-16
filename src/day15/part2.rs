use std::{
    collections::{HashMap, HashSet},
    usize,
};

const EMPTY: char = '.';
const BOX: char = 'O';
const BOX0: char = '[';
const BOX1: char = ']';
const WALL: char = '#';
const ROBOT: char = '@';

pub enum MoveDir {
    Up,
    Right,
    Down,
    Left,
}

impl MoveDir {
    pub fn apply_to(&self, pos: Position) -> Position {
        match pos {
            Position::Lower(j, i) => match self {
                MoveDir::Up => Position::Lower(j - 1, i),
                MoveDir::Right => Position::Upper(j, i),
                MoveDir::Down => Position::Lower(j + 1, i),
                MoveDir::Left => Position::Upper(j, i - 1),
            },
            Position::Upper(j, i) => match self {
                MoveDir::Up => Position::Upper(j - 1, i),
                MoveDir::Right => Position::Lower(j, i + 1),
                MoveDir::Down => Position::Upper(j + 1, i),
                MoveDir::Left => Position::Lower(j, i),
            },
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
enum Position {
    Lower(usize, usize),
    Upper(usize, usize),
}

impl Position {
    /* fn to_old_storage_system(&self) -> (usize, usize) {
        match self {
            Position::Lower(j, i) => (*j, *i),
            Position::Upper(j, i) => (*j, *i),
        }
    }
    fn to_new_storage_system(&self) -> (usize, usize) {
        match self {
            Position::Lower(j, i) => (*j, 2 * i),
            Position::Upper(j, i) => (*j, 2 * i + 1),
        }
    }*/
}

#[allow(dead_code)]
pub struct StorageState {
    height: usize,
    width: usize,
    boxes_lower: HashMap<usize, Position>,
    walls_lower: HashSet<Position>,
    robot_pos: Position,
}

impl StorageState {
    pub fn is_wall(&self, at: Position) -> bool {
        if self
            .walls_lower
            .iter()
            .any(|p| *p == at || MoveDir::Right.apply_to(*p) == at)
        {
            return true;
        }
        false
    }
    pub fn get_box(&self, at: Position) -> Option<usize> {
        self.boxes_lower
            .iter()
            .find(|&(_, pos)| *pos == at || MoveDir::Right.apply_to(*pos) == at)
            .map(|e| *e.0)
    }
    fn move_if_possible(&mut self, pos: Position, dir: &MoveDir) -> Option<Position> {
        let dest = dir.apply_to(pos);

        if self.is_wall(dest) {
            return None;
        }

        if let Some(id) = self.get_box(dest) {
            if let Some(_) = self.move_if_possible(dest, dir) {
                let box_lower_pos = *self.boxes_lower.get(&id).unwrap();
                *self.boxes_lower.get_mut(&id).unwrap() = dir.apply_to(box_lower_pos);
                Some(dest)
            } else {
                None
            }
        } else {
            Some(dest)
        }
    }

    #[allow(unused)]
    pub fn run_robot_movements(&mut self, movements: &[MoveDir]) -> usize {
        for (k, dir) in movements.iter().enumerate() {
            if let Some(next_robot_pos) = self.move_if_possible(self.robot_pos, dir) {
                self.robot_pos = next_robot_pos;
            }
            //println!("\nafter {k} moves:\n");
            //self.print();
        }

        self.calc_gps_sum()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for j in 0..self.height {
            let mut row = String::with_capacity(self.width);
            for i in 0..self.width {
                if self.is_wall(Position::Lower(j, i)) {
                    row.push(WALL);
                    row.push(WALL);
                } else if let Some(id) = self.get_box(Position::Lower(j, i)) {
                    match self.boxes_lower.get(&id).unwrap() {
                        Position::Lower(j, i) => {
                            row.push(BOX0);
                            row.push(BOX1);
                        }
                        Position::Upper(j, i) => {
                            row.push(BOX1);
                            row.push(BOX0);
                        }
                    }
                } else if self.robot_pos == Position::Lower(j, i) {
                    row.push(ROBOT);
                    row.push(EMPTY);
                } else {
                    row.push(EMPTY);
                    row.push(EMPTY);
                }
            }
            println!("{row}");
        }
    }

    pub fn calc_gps_sum(&self) -> usize {
        let mut s = 0;
        for p in self.boxes_lower.values() {
            let (j, i) = match p {
                Position::Lower(j, i) => (j, 2 * i),
                Position::Upper(j, i) => (j, 2 * i + 1),
            };
            s += j * 100 + i;
        }
        s
    }
}

pub fn parse_input(input: &str) -> (StorageState, Vec<MoveDir>) {
    let mut lines = input.lines();

    let mut walls_lower = HashSet::new();
    let mut boxes_lower = HashMap::new();
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
                    boxes_lower.insert(next_box_id, Position::Lower(j, i));
                    next_box_id += 1;
                }
                WALL => {
                    walls_lower.insert(Position::Lower(j, i));
                }
                ROBOT => {
                    robot_pos = Some(Position::Lower(j, i));
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
            walls_lower,
            boxes_lower,
            robot_pos: robot_pos.unwrap(),
        },
        movements,
    )
}

pub fn part2(input: &str) -> usize {
    let (mut storage_state, movements) = parse_input(input);

    storage_state.run_robot_movements(&movements[0..1])
}
