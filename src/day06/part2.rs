use std::collections::HashSet;

use crate::day06::InputStats;

use super::{parse_input, process_visits, Orientation};

fn check_for_potential_obstruction(pos: (usize, usize), mut input_stats: InputStats) -> bool {
    input_stats.add_obstacle(pos);
    let mut tracking = HashSet::<((usize, usize), Orientation)>::new();

    let mut is_potential_obstruction = false;
    process_visits(input_stats, |pos, orientation| {
        if tracking.contains(&(pos, orientation)) {
            is_potential_obstruction = true;
            false
        } else {
            tracking.insert((pos, orientation));
            true
        }
    });

    is_potential_obstruction
}

pub fn part2(input: &str) -> usize {
    //let mut visited_with_orientation = Vec::<PositionWithOrientation>::new();
    let mut visited = HashSet::<(usize, usize)>::new();

    let input_stats = parse_input(input);
    process_visits(input_stats.clone(), |pos, _| {
        visited.insert(pos);
        true
    });

    // Now we have to check each visited position (others are irrelevant) in regard of being a potential obstruction for enforcing a loop
    let mut obstructions = HashSet::<(usize, usize)>::new();

    for p in visited {
        if check_for_potential_obstruction(p, input_stats.clone()) {
            obstructions.insert(p);
        }
    }

    /*
       for k in 1..visited_with_orientation.len() {
           if obstructions.contains(&visited_with_orientation[k].0) {
               continue;
           }
           let prev = visited_with_orientation[k - 1];

           let orientation_if_obstruction_at_cur_pos = prev.1.turn_right();

           let mut potential_pos = prev.0;
           loop {
               potential_pos =
           }

           let pos_if_obstruction_at_cur_pos = orientation_if_obstruction_at_cur_pos.step_from(prev.0);
           if visited_with_orientation[0..k].contains(&(
               pos_if_obstruction_at_cur_pos,
               orientation_if_obstruction_at_cur_pos,
           )) {
               println!("put obstruction at {:?}", visited_with_orientation[k].0);
               obstructions.insert(visited_with_orientation[k].0);
           }
       }
    */
    obstructions.len()
}
