use std::collections::HashMap;
mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

fn diff(p1: (i64, i64), p0: (i64, i64), normalized: bool) -> (i64, i64) {
    let diff = (p1.0 - p0.0, p1.1 - p0.1);
    if normalized {
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
    }
    diff
}

type InputStats = (usize, usize, HashMap<char, Vec<(i64, i64)>>);

fn parse_antenna_maps(input: &str) -> InputStats {
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

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const PART1_OUTPUT: usize = 14;

    #[test]
    fn day08_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 34;

    #[test]
    fn day08_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
