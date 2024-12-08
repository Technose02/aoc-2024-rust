use std::{
    collections::{HashMap, HashSet},
    usize,
};

fn parse_input(input: &str) -> (usize, usize, HashMap<char, Vec<(i64, i64)>>) {
    let mut antennas_by_frequency = HashMap::<char, Vec<(i64, i64)>>::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != '.' {
                if let Some(m) = antennas_by_frequency.get_mut(&c) {
                    m.push((j as i64, i as i64));
                } else {
                    antennas_by_frequency.insert(c, vec![(j as i64, i as i64)]);
                }
            }
        }
    }
    (height, width, antennas_by_frequency)
}

fn diff(p1: (i64, i64), p0: (i64, i64)) -> (i64, i64) {
    (p1.0 - p0.0, p1.1 - p0.1)
}

fn dist(p0: (i64, i64), p1: (i64, i64)) -> i64 {
    let d = diff(p0, p1);
    (d.0.abs() + d.1.abs()) as i64
}

fn normalize_diff(diff: (i64, i64)) -> (i64, i64) {
    let x = if diff.0.abs() <= diff.1.abs() {
        diff.0
    } else {
        diff.1
    };
    for d in (2..=x.abs()).rev() {
        if diff.0 % d == 0 && diff.1 % d == 0 {
            return (diff.0 / d, diff.1 / d);
        }
    }
    diff
}

pub fn part1(input: &str) -> usize {
    let (height, width, antennas_by_frequency) = parse_input(input);
    println!("height: {height}, width: {width}");

    let mut antinodes = HashSet::<(i64, i64)>::new();

    for (frequency, antenna_positions) in &antennas_by_frequency {
        for idx0 in 0..antenna_positions.len() - 1 {
            for idx1 in idx0 + 1..antenna_positions.len() {
                let p0 = antenna_positions[idx0];
                let p1 = antenna_positions[idx1];

                println!(
                    "checking for antinodes of '{frequency}'-antennas at ({},{}) and ({},{})...",
                    p0.0, p0.1, p1.0, p1.1
                );

                let dir10: (i64, i64) = normalize_diff(diff(p0, p1));
                println!("\tnormalized dir-vec for iterating from ({},{}) increasing the distance to ({},{}): ({},{})", p0.0,p0.1,p1.0,p1.1,dir10.0,dir10.1);

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
                        println!("\tinserting antinode at ({},{})", pos.0, pos.1);
                        antinodes.insert(pos);
                    }
                    //if d0 > dd1 {
                    //    break;
                    //}
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
                        println!("\tinserting antinode at ({},{})", pos.0, pos.1);
                        antinodes.insert(pos);
                    }
                    //if d0 > dd1 {
                    //    break;
                    //}
                }

                let dir01: (i64, i64) = normalize_diff(diff(p1, p0));
                println!("\tnormalized dir-vec for iterating from ({},{}) increasing the distance to ({},{}): ({},{})", p1.0,p1.1,p0.0,p0.1,dir01.0,dir01.1);

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
                        println!("\tinserting antinode at ({},{})", pos.0, pos.1);
                        antinodes.insert(pos);
                    }
                    //if d0 > dd1 {
                    //    break;
                    //}
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
                        println!("\tinserting antinode at ({},{})", pos.0, pos.1);
                        antinodes.insert(pos);
                    }
                    //if d0 > dd1 {
                    //    break;
                    //}
                }
            }
        }
    }

    /*
        for j in 0..height {
            for i in 0..width {
                println!("(j,i): ({j},{i})");
                for (frequency, antenna_positions) in &antennas_by_frequency {
                    let mut dist_matches = HashSet::<usize>::new();
                    for pos in antenna_positions {
                        let d = dist((j, i), (pos.0, pos.1));
                        dist_matches.insert(d);
                        println!("\tregistered antenna for frequency '{frequency}' at ({},{}) with distance {d} to ({j},{i})", pos.0,pos.1);
                    }
                    for pos in antenna_positions {
                        let d = dist((j, i), (pos.0, pos.1));
                        if d % 2 == 0 {
                            let half_distance = d / 2;
                            if dist_matches.contains(&half_distance) {
                                println!("setting antinode at '({j},{i})' for frequency '{frequency}'");
                                antinodes.insert((j, i));
                            }
                        }
                    }
                }
                println!("==================");
            }
        }
    */
    println!("{antennas_by_frequency:?}");
    println!("{antinodes:?}");
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
