use crate::util::ThreadPool;

use super::Map;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let borrowed_map = Arc::new(map);

    let sum = Arc::new(Mutex::new(0_usize));

    {
        let potential_trail_heads = borrowed_map.potential_trails();
        let tp = ThreadPool::new(8);
        for head in potential_trail_heads {
            let sum = sum.clone();
            let borrowed_map = borrowed_map.clone();
            tp.execute(move || {
                let mut m = HashSet::<(usize, usize)>::new();
                m.extend(borrowed_map.check_subtrail_start(head));
                *sum.lock().unwrap() += m.len()
            });
        }
    }

    *sum.clone().lock().unwrap()
}
