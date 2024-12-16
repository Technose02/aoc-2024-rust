use std::collections::{HashMap, HashSet};

const EMPTY: char = '.';
const BOX: char = 'O';
const BOX0: char = '[';
const BOX1: char = ']';
const WALL: char = '#';
const ROBOT: char = '@';

#[derive(Debug, PartialEq, Clone, Copy)]
struct Box(usize, usize, usize);

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Wall(usize, usize, usize);

#[derive(Debug, PartialEq)]
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
    boxes: HashMap<usize, Box>,
    walls: HashSet<Wall>,
    robot_pos: (usize, usize),
}

impl StorageState {
    //pub fn is_wall_lower(&self, at: Position) -> bool {
    //    if self.walls_lower.iter().any(|p| *p == at) {
    //        return true;
    //    }
    //    false
    //}
    //pub fn is_wall_upper(&self, at: Position) -> bool {
    //    if self
    //        .walls_lower
    //        .iter()
    //        .any(|p| MoveDir::Right.apply_to(*p) == at)
    //    {
    //        return true;
    //    }
    //    false
    //}
    //
    //pub fn get_box_lower(&self, at: Position) -> Option<usize> {
    //    self.boxes_lower
    //        .iter()
    //        .find(|&(_, pos)| *pos == at)
    //        .map(|e| *e.0)
    //}
    //
    //pub fn get_box_upper(&self, at: Position) -> Option<usize> {
    //    self.boxes_lower
    //        .iter()
    //        .find(|&(_, pos)| MoveDir::Right.apply_to(*pos) == at)
    //        .map(|e| *e.0)
    //}

    fn move_if_possible(&mut self, pos: (usize, usize), dir: &MoveDir) -> Option<(usize, usize)> {
        let dest = dir.apply_to(pos);

        if self.is_wall(dest) {
            return None;
        }
        Some(dest) // wrong!

        /*
        if self.is_wall_lower(dest) || self.is_wall_upper(dest) {
            return None;
        }

        if let Some(id) = self.get_box_lower(dest) {
            if *dir == MoveDir::Left || *dir == MoveDir::Right {
                if let Some(_) = self.move_if_possible(dir.apply_to(dest), dir) {
                    println!("must move box {id} in dir '{dir:?}'");
                    let box_lower_pos = *self.boxes_lower.get(&id).unwrap();
                    *self.boxes_lower.get_mut(&id).unwrap() = dir.apply_to(box_lower_pos);
                    Some(dest)
                } else {
                    None
                }
            } else {
                if let Some(_) = self.move_if_possible(dest, dir) {
                    println!("must move box {id} in dir '{dir:?}'");
                    let box_lower_pos = *self.boxes_lower.get(&id).unwrap();
                    *self.boxes_lower.get_mut(&id).unwrap() = dir.apply_to(box_lower_pos);
                    Some(dest)
                } else {
                    None
                }
            }
        } else if let Some(id) = self.get_box_upper(dest) {
            if let Some(_) = self.move_if_possible(dir.apply_to(dest), dir) {
                println!("must move box {id} in dir '{dir:?}'");
                let box_lower_pos = *self.boxes_lower.get(&id).unwrap();
                *self.boxes_lower.get_mut(&id).unwrap() = dir.apply_to(box_lower_pos);
                Some(dest)
            } else {
                None
            }
        } else {
            Some(dest)
        }*/
    }

    #[allow(unused)]
    pub fn run_robot_movements(&mut self, movements: &[MoveDir]) -> usize {
        /*for (k, dir) in movements.iter().enumerate() {
            if let Some(next_robot_pos) = self.move_if_possible(self.robot_pos, dir) {
                self.robot_pos = next_robot_pos;
            }
            println!("\nafter {} moves:\n", k + 1);
            println!("{:?}", self.boxes_lower);
            self.print();
        }

        self.calc_gps_sum()*/
        0
    }

    fn is_wall(&self, at: (usize, usize)) -> bool {
        self.walls
            .iter()
            .any(|&Wall(j, i1, i2)| j == at.0 && (i1 == at.1 || i2 == at.1))
    }

    fn is_box0(&self, at: (usize, usize)) -> bool {
        self.boxes
            .values()
            .any(|&Box(j, i, _)| j == at.0 && i == at.1)
    }
    fn is_box1(&self, at: (usize, usize)) -> bool {
        self.boxes
            .values()
            .any(|&Box(j, _, i)| j == at.0 && i == at.1)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for j in 0..self.height {
            let mut row = String::with_capacity(self.width);
            for i in 0..self.width {
                if self.is_wall((j, i)) {
                    row.push(WALL);
                } else if self.is_box0((j, i)) {
                    row.push(BOX0);
                } else if self.is_box1((j, i)) {
                    row.push(BOX1);
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
        /*let mut s = 0;
        for p in self.boxes_lower.values() {
            let (j, i) = match p {
                Position::Lower(j, i) => (j, 2 * i),
                Position::Upper(j, i) => (j, 2 * i + 1),
            };
            s += j * 100 + i;
        }
        s*/
        0
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
                    boxes.insert(next_box_id, Box(j, 2 * i, 2 * i + 1));
                    next_box_id += 1;
                }
                WALL => {
                    walls.insert(Wall(j, 2 * i, 2 * i + 1));
                }
                ROBOT => {
                    robot_pos = Some((j, 2 * i));
                }
                EMPTY => (),
                _ => panic!(
                    "unexpected character '{c}' encountered while parsing initial StorageState"
                ),
            }
            i += 1;
        }
        if width.is_none() {
            width = Some(2 * i);
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

    let storage_state = StorageState {
        height: j,
        width: width.unwrap(),
        walls,
        boxes,
        robot_pos: robot_pos.unwrap(),
    };
    println!("initial state:");
    storage_state.print();

    (storage_state, movements)
}

pub fn part2(input: &str) -> usize {
    let (mut storage_state, movements) = parse_input(input);

    println!("walls: {:?}", storage_state.walls);
    println!("boxes: {:?}", storage_state.boxes);

    storage_state.run_robot_movements(&movements[0..])
    //storage_state.run_robot_movements(&movements)
}