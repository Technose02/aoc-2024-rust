use super::{diff, parse_antenna_maps};
use std::{collections::HashSet, ops::Neg};

fn collect_positions(
    start: (i64, i64),
    dir: (i64, i64),
    lower_limit: (i64, i64),
    upper_limit: (i64, i64),
    set: &mut HashSet<(i64, i64)>,
) {
    let mut iter_pos = start;
    loop {
        iter_pos = (iter_pos.0 + dir.0, iter_pos.1 + dir.1);
        if !(lower_limit.0..=upper_limit.0).contains(&iter_pos.0)
            || !(lower_limit.1..=upper_limit.1).contains(&iter_pos.1)
        {
            break;
        }
        set.insert(iter_pos);
    }
}

fn collect_grid_positions_in_line(
    p0: (i64, i64),
    p1: (i64, i64),
    width: i64,
    height: i64,
    set: &mut HashSet<(i64, i64)>,
) {
    let dir = diff(p0, p1, true);
    set.insert(p0);
    collect_positions(p0, dir, (0, 0), (height - 1, width - 1), set);
    collect_positions(
        p0,
        (dir.0.neg(), dir.1.neg()),
        (0, 0),
        (height - 1, width - 1),
        set,
    );
}

pub fn part2(input: &str) -> usize {
    let (height, width, antennas_by_frequency) = parse_antenna_maps(input);

    let mut antinodes = HashSet::<(i64, i64)>::new();
    {
        for antenna_positions in antennas_by_frequency.values() {
            let antenna_positions = antenna_positions.clone();
            for idx0 in 0..antenna_positions.len() - 1 {
                for idx1 in idx0 + 1..antenna_positions.len() {
                    let p0 = antenna_positions[idx0];
                    let p1 = antenna_positions[idx1];

                    collect_grid_positions_in_line(
                        p0,
                        p1,
                        width as i64,
                        height as i64,
                        &mut antinodes,
                    );
                }
            }
        }
    }

    antinodes.len()
}
