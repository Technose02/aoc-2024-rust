use crate::util::Map;
use std::collections::HashSet;

fn region_from(
    plant_type: &char,
    pos: (usize, usize),
    map: &Map<char>,
    visited: &mut HashSet<(usize, usize)>,
    area: &mut usize,
    perimeter: &mut usize,
) {
    if visited.contains(&pos) {
        return;
    }

    visited.insert(pos);
    *area += 1;

    // TOP
    if pos.0 > 0 {
        let nghbr = (pos.0 - 1, pos.1);
        if map.at(nghbr) == *plant_type {
            region_from(plant_type, nghbr, map, visited, area, perimeter)
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }

    // RIGHT
    if pos.1 < map.width() - 1 {
        let nghbr = (pos.0, pos.1 + 1);
        if map.at(nghbr) == *plant_type {
            region_from(plant_type, nghbr, map, visited, area, perimeter)
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }

    // BOTTOM
    if pos.0 < map.height() - 1 {
        let nghbr = (pos.0 + 1, pos.1);
        if map.at(nghbr) == *plant_type {
            region_from(plant_type, nghbr, map, visited, area, perimeter)
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }
    // LEFT
    if pos.1 > 0 {
        let nghbr = (pos.0, pos.1 - 1);
        if map.at(nghbr) == *plant_type {
            region_from(plant_type, nghbr, map, visited, area, perimeter)
        } else {
            *perimeter += 1;
        }
    } else {
        *perimeter += 1;
    }
}

pub fn part1(input: &str) -> usize {
    let map = Map::parse(input);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut fences_costs = 0;
    for (j, row) in map.rows().enumerate() {
        for (i, plant_type) in row.iter().enumerate() {
            if visited.contains(&(j, i)) {
                continue;
            }

            let mut area = 0_usize;
            let mut perimeter = 0_usize;
            region_from(
                plant_type,
                (j, i),
                &map,
                &mut visited,
                &mut area,
                &mut perimeter,
            );
            //println!("discovered '{plant_type}'-region of area {area} and perimeter {perimeter}");
            fences_costs += area * perimeter;
        }
    }

    fences_costs
}
