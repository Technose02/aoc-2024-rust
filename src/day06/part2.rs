use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::{day06::InputStats, util::ThreadPool};

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
    let obstructions = Arc::new(Mutex::new(HashSet::<(usize, usize)>::new()));

    {
        let pool = ThreadPool::new(16);
        for p in visited {
            {
                let p = p.to_owned();
                let input_stats = input_stats.clone();
                let obstructions = obstructions.clone();
                pool.execute(move || {
                    if check_for_potential_obstruction(p, input_stats.clone()) {
                        obstructions.lock().unwrap().insert(p);
                    }
                });
            }
        }
    }
    obstructions.clone().lock().unwrap().len()
}
