use super::{diff, parse_antenna_maps};
use std::{collections::HashSet, ops::Neg};

fn dist(p0: (i64, i64), p1: (i64, i64)) -> i64 {
    let d = diff(p0, p1, false);
    (d.0.abs() + d.1.abs()) as i64
}

fn collect_grid_positions_in_line(
    p0: (i64, i64),
    p1: (i64, i64),
    width: i64,
    height: i64,
    set: &mut HashSet<(i64, i64)>,
) {
    let dir = diff(p0, p1, true);
    //set.insert(p0);

    ///////////
    let mut iter_pos = p0;
    loop {
        iter_pos = (iter_pos.0 + dir.0, iter_pos.1 + dir.1);
        if !(0..height).contains(&iter_pos.0) || !(0..width).contains(&iter_pos.1) {
            break;
        }
        let d0 = dist(p0, iter_pos);
        let d1 = dist(p1, iter_pos);
        if d0 == 2 * d1 || 2 * d0 == d1 {
            set.insert(iter_pos);
        }
    }
    ///////////

    ///////////
    iter_pos = p0;
    let dir = (dir.0.neg(), dir.1.neg());
    loop {
        iter_pos = (iter_pos.0 + dir.0, iter_pos.1 + dir.1);
        if !(0..height).contains(&iter_pos.0) || !(0..width).contains(&iter_pos.1) {
            break;
        }
        let d0 = dist(p0, iter_pos);
        let d1 = dist(p1, iter_pos);
        if d0 == 2 * d1 || 2 * d0 == d1 {
            set.insert(iter_pos);
        }
    }
    ///////////
}

pub fn part1(input: &str) -> usize {
    let (height, width, antennas_by_frequency) = parse_antenna_maps(input);

    let mut antinodes = HashSet::<(i64, i64)>::new();

    for antenna_positions in antennas_by_frequency.values() {
        for idx0 in 0..antenna_positions.len() - 1 {
            for idx1 in idx0 + 1..antenna_positions.len() {
                let p0 = antenna_positions[idx0];
                let p1 = antenna_positions[idx1];

                collect_grid_positions_in_line(p0, p1, width as i64, height as i64, &mut antinodes);
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn normalized_diff_should_work() {
        assert_eq!((1, 5), diff((1, 5), (0, 0), true));
        assert_eq!((1, 5), diff((2, 10), (0, 0), true));
        assert_eq!((3, 5), diff((3, 5), (0, 0), true));
        assert_eq!((-1, 7), diff((-1, 7), (0, 0), true));
        assert_eq!((-2, 7), diff((-2, 7), (0, 0), true));
        assert_eq!((-1, 4), diff((-2, 8), (0, 0), true));
        assert_eq!((1, -4), diff((2, -8), (0, 0), true));
        assert_eq!((1, -7), diff((1, -7), (0, 0), true));
        assert_eq!((2, -7), diff((2, -7), (0, 0), true));
    }
}
