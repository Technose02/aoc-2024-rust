use crate::util::Map;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct BoundaryPositions {
    top: HashSet<(usize, usize)>,
    right: HashSet<(usize, usize)>,
    bottom: HashSet<(usize, usize)>,
    left: HashSet<(usize, usize)>,
}

impl BoundaryPositions {
    pub fn add_top(&mut self, pos: (usize, usize)) {
        self.top.insert(pos);
    }
    pub fn add_right(&mut self, pos: (usize, usize)) {
        self.right.insert(pos);
    }
    pub fn add_bottom(&mut self, pos: (usize, usize)) {
        self.bottom.insert(pos);
    }
    pub fn add_left(&mut self, pos: (usize, usize)) {
        self.left.insert(pos);
    }

    fn count_horizontal_sides(&self) -> usize {
        let mut count = 0;
        for set in &[&self.top, &self.bottom] {
            let mut m = HashMap::<usize, Vec<usize>>::new();
            for p in *set {
                if let Some(v) = m.get_mut(&p.0) {
                    v.push(p.1);
                } else {
                    m.insert(p.0, vec![p.1]);
                }
            }
            for (_, v) in m.iter_mut() {
                count += 1;
                v.sort();
                for i in 1..v.len() {
                    if v[i].abs_diff(v[i - 1]) > 1 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn count_vertical_sides(&self) -> usize {
        let mut count = 0;
        for set in &[&self.right, &self.left] {
            let mut m = HashMap::<usize, Vec<usize>>::new();
            for p in *set {
                if let Some(v) = m.get_mut(&p.1) {
                    v.push(p.0);
                } else {
                    m.insert(p.1, vec![p.0]);
                }
            }
            for (_, v) in m.iter_mut() {
                count += 1;
                v.sort();
                for i in 1..v.len() {
                    if v[i].abs_diff(v[i - 1]) > 1 {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn sides_count(&self) -> usize {
        self.count_horizontal_sides() + self.count_vertical_sides()
    }
}

fn region_from(
    plant_type: &char,
    pos: (usize, usize),
    map: &Map<char>,
    visited: &mut HashSet<(usize, usize)>,
    area: &mut usize,
    boundary_positions: &mut BoundaryPositions,
) {
    if visited.contains(&pos) {
        return;
    }

    visited.insert(pos);
    *area += 1;

    let (mut top_nghbr, mut right_nghbr, mut bottom_nghbr, mut left_nghbr) =
        (None, None, None, None);

    if pos.0 > 0 {
        let nghbr = (pos.0 - 1, pos.1);
        if map.at(nghbr) == *plant_type {
            top_nghbr = Some(nghbr);
        }
    }
    if pos.1 < map.width() - 1 {
        let nghbr = (pos.0, pos.1 + 1);
        if map.at(nghbr) == *plant_type {
            right_nghbr = Some(nghbr);
        }
    }
    if pos.0 < map.height() - 1 {
        let nghbr = (pos.0 + 1, pos.1);
        if map.at(nghbr) == *plant_type {
            bottom_nghbr = Some(nghbr);
        }
    }
    if pos.1 > 0 {
        let nghbr = (pos.0, pos.1 - 1);
        if map.at(nghbr) == *plant_type {
            left_nghbr = Some(nghbr);
        }
    }

    // TOP
    if let Some(pos) = top_nghbr {
        region_from(plant_type, pos, map, visited, area, boundary_positions)
    } else {
        boundary_positions.add_top(pos);
    }
    // RIGHT
    if let Some(pos) = right_nghbr {
        region_from(plant_type, pos, map, visited, area, boundary_positions)
    } else {
        boundary_positions.add_right(pos);
    }
    // BOTTOM
    if let Some(pos) = bottom_nghbr {
        region_from(plant_type, pos, map, visited, area, boundary_positions)
    } else {
        boundary_positions.add_bottom(pos);
    }
    // LEFT
    if let Some(pos) = left_nghbr {
        region_from(plant_type, pos, map, visited, area, boundary_positions)
    } else {
        boundary_positions.add_left(pos);
    }
}

pub fn part2(input: &str) -> usize {
    let map: Map<char> = Map::parse(input);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut fences_costs = 0;
    for (j, row) in map.rows().enumerate() {
        for (i, plant_type) in row.iter().enumerate() {
            if visited.contains(&(j, i)) {
                continue;
            }

            let mut area = 0_usize;
            let mut boundary_positions = BoundaryPositions::default();
            region_from(
                plant_type,
                (j, i),
                &map,
                &mut visited,
                &mut area,
                &mut boundary_positions,
            );
            //println!("discovered '{plant_type}'-region of area {area} and boundaries at:");
            //println!("\t{boundary_positions:?}");
            //println!(
            //    "\t\t-> horizontal sides: {}",
            //    boundary_positions.count_horizontal_sides()
            //);
            //println!(
            //    "\t\t-> vertical sides: {}",
            //    boundary_positions.count_vertical_sides()
            //);
            //println!("\t\t-> total sides: {}", boundary_positions.sides_count());
            fences_costs += area * boundary_positions.sides_count();
        }
    }

    fences_costs
}
