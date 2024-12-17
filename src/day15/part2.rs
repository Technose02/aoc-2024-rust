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
    fn can_move_box(
        &self,
        dir: &MoveDir,
        box_id: usize,
        affected_boxes: &mut HashSet<usize>,
    ) -> bool {
        let dest = {
            let b = self.boxes.get(&box_id).expect("illegal box_id provided");
            let dest0 = dir.apply_to((b.0, b.1));
            let dest1 = dir.apply_to((b.0, b.2));
            (dest0.0, dest0.1, dest1.1)
        };

        if self.is_wall((dest.0, dest.1)) || self.is_wall((dest.0, dest.2)) {
            return false;
        }

        affected_boxes.insert(box_id);

        match *dir {
            MoveDir::Left => {
                if let Some(id) = self.get_box1((dest.0, dest.1)) {
                    // a box in move-direction to the left
                    return self.can_move_box(dir, id, affected_boxes);
                }
            }
            MoveDir::Right => {
                if let Some(id) = self.get_box0((dest.0, dest.2)) {
                    // a box in move-direction to the right
                    return self.can_move_box(dir, id, affected_boxes);
                }
            }
            MoveDir::Up | MoveDir::Down => {
                if let Some(id) = self.get_box0((dest.0, dest.1)) {
                    // lined up with exactly one box
                    return self.can_move_box(dir, id, affected_boxes);
                }

                if let Some(id_a) = self.get_box1((dest.0, dest.1)) {
                    if let Some(id_b) = self.get_box0((dest.0, dest.2)) {
                        // one box to the left and one box to the right affected

                        return self.can_move_box(dir, id_a, affected_boxes)
                            && self.can_move_box(dir, id_b, affected_boxes);
                    } else {
                        // only one box to the left affected
                        return self.can_move_box(dir, id_a, affected_boxes);
                    }
                }

                if let Some(id_b) = self.get_box0((dest.0, dest.2)) {
                    // only one box to the right affected
                    return self.can_move_box(dir, id_b, affected_boxes);
                }
            }
        }

        // actually the box is free to move
        true
    }

    fn move_robot_if_possible(&mut self, dir: &MoveDir) -> bool {
        let dest = dir.apply_to(self.robot_pos);
        if self.is_wall(dest) {
            return false;
        } else if let Some(id0) = self.get_box0(dest) {
            let mut affected_boxes = HashSet::new();
            if self.can_move_box(dir, id0, &mut affected_boxes) {
                for affected_box in affected_boxes {
                    let val: &mut Box = self.boxes.get_mut(&affected_box).unwrap();
                    let v1 = dir.apply_to((val.0, val.1));
                    let v2 = dir.apply_to((val.0, val.2));
                    *val = Box(v1.0, v1.1, v2.1);
                }
                self.robot_pos = dest;
                return true;
            } else {
                return false;
            }
        } else if let Some(id1) = self.get_box1(dest) {
            let mut affected_boxes = HashSet::new();
            if self.can_move_box(dir, id1, &mut affected_boxes) {
                for affected_box in affected_boxes {
                    let val: &mut Box = self.boxes.get_mut(&affected_box).unwrap();
                    let v1 = dir.apply_to((val.0, val.1));
                    let v2 = dir.apply_to((val.0, val.2));
                    *val = Box(v1.0, v1.1, v2.1);
                }
                self.robot_pos = dest;
                return true;
            } else {
                return false;
            }
        }
        self.robot_pos = dest;
        true
    }

    #[allow(unused)]
    pub fn run_robot_movements(&mut self, movements: &[MoveDir]) -> usize {
        for (k, dir) in movements.iter().enumerate() {
            self.move_robot_if_possible(dir);
            //println!("\nafter {} moves:\n", _k + 1);
            //self.print();
        }

        self.calc_gps_sum()
    }

    fn is_wall(&self, at: (usize, usize)) -> bool {
        self.walls
            .iter()
            .any(|&Wall(j, i1, i2)| j == at.0 && (i1 == at.1 || i2 == at.1))
    }

    fn get_box0(&self, at: (usize, usize)) -> Option<usize> {
        self.boxes
            .iter()
            .find(|(_, Box(j, i, _))| *j == at.0 && *i == at.1)
            .map(|(&id, _)| id)
    }

    fn get_box1(&self, at: (usize, usize)) -> Option<usize> {
        self.boxes
            .iter()
            .find(|(_, Box(j, _, i))| *j == at.0 && *i == at.1)
            .map(|(&id, _)| id)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for j in 0..self.height {
            let mut row = String::with_capacity(self.width);
            for i in 0..self.width {
                if self.is_wall((j, i)) {
                    row.push(WALL);
                } else if self.get_box0((j, i)).is_some() {
                    row.push(BOX0);
                } else if self.get_box1((j, i)).is_some() {
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
        let mut s = 0;
        for Box(j, i, _) in self.boxes.values() {
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
    //println!("initial state:");
    //storage_state.print();

    (storage_state, movements)
}

pub fn part2(input: &str) -> usize {
    let (mut storage_state, movements) = parse_input(input);

    storage_state.run_robot_movements(&movements)
}
