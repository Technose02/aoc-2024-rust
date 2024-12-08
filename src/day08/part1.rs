use super::{diff, normalize_diff, parse_antenna_maps};
use std::{collections::HashSet, usize};

fn dist(p0: (i64, i64), p1: (i64, i64)) -> i64 {
    let d = diff(p0, p1);
    (d.0.abs() + d.1.abs()) as i64
}

pub fn part1(input: &str) -> usize {
    let (height, width, antennas_by_frequency) = parse_antenna_maps(input);

    let mut antinodes = HashSet::<(i64, i64)>::new();

    for antenna_positions in antennas_by_frequency.values() {
        for idx0 in 0..antenna_positions.len() - 1 {
            for idx1 in idx0 + 1..antenna_positions.len() {
                let p0 = antenna_positions[idx0];
                let p1 = antenna_positions[idx1];

                let dir10: (i64, i64) = normalize_diff(diff(p0, p1));

                let mut step = 0_i64;
                loop {
                    step += 1;
                    let pos = (p0.0 + step * dir10.0, p0.1 + step * dir10.1);
                    if pos.0 < 0 || pos.1 < 0 || pos.0 >= height as i64 || pos.1 >= width as i64 {
                        break;
                    }
                    let d0 = dist(p0, pos);
                    let d1 = dist(p1, pos);
                    if d0 == 2 * d1 || 2 * d0 == d1 {
                        antinodes.insert(pos);
                    }
                }
                let mut step = 0_i64;
                loop {
                    step -= 1;
                    let pos = (p0.0 + step * dir10.0, p0.1 + step * dir10.1);
                    if pos.0 < 0 || pos.1 < 0 || pos.0 >= height as i64 || pos.1 >= width as i64 {
                        break;
                    }
                    let d0 = dist(p0, pos);
                    let d1 = dist(p1, pos);
                    if d0 == 2 * d1 || 2 * d0 == d1 {
                        antinodes.insert(pos);
                    }
                }

                let dir01: (i64, i64) = normalize_diff(diff(p1, p0));

                step = 0;
                loop {
                    step += 1;
                    let pos = (p1.0 + step * dir01.0, p1.1 + step * dir01.1);
                    if pos.0 < 0 || pos.1 < 0 || pos.0 >= height as i64 || pos.1 >= width as i64 {
                        break;
                    }
                    let d0 = dist(p0, pos);
                    let d1 = dist(p1, pos);
                    if d1 == 2 * d0 || d0 == 2 * d1 {
                        antinodes.insert(pos);
                    }
                }
                step = 0;
                loop {
                    step -= 1;
                    let pos = (p1.0 + step * dir01.0, p1.1 + step * dir01.1);
                    if pos.0 < 0 || pos.1 < 0 || pos.0 >= height as i64 || pos.1 >= width as i64 {
                        break;
                    }
                    let d0 = dist(p0, pos);
                    let d1 = dist(p1, pos);
                    if d1 == 2 * d0 || d0 == 2 * d1 {
                        antinodes.insert(pos);
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn normalize_diff_should_work() {
        assert_eq!((1, 5), normalize_diff((1, 5)));
        assert_eq!((1, 5), normalize_diff((2, 10)));
        assert_eq!((3, 5), normalize_diff((3, 5)));
        assert_eq!((-1, 7), normalize_diff((-1, 7)));
        assert_eq!((-2, 7), normalize_diff((-2, 7)));
        assert_eq!((-1, 4), normalize_diff((-2, 8)));
        assert_eq!((1, -4), normalize_diff((2, -8)));
        assert_eq!((1, -7), normalize_diff((1, -7)));
        assert_eq!((2, -7), normalize_diff((2, -7)));
    }
}
