use super::{parse_input, Robot, RobotPosition};
use crate::util::Map;

const EMPTY: char = ' ';
const SET: char = 'o';

fn get_frame<const HEIGHT: usize, const WIDTH: usize>(robots: &Vec<Robot>, n: usize) -> Map<char> {
    let mut m = Map::new(HEIGHT, WIDTH, EMPTY);

    for robot in robots {
        let RobotPosition { x, y } = robot.predict_future_position(n, HEIGHT, WIDTH);
        m.set_at((y, x), SET);
    }

    m
}

fn check_frame_for_christmas_tree(frame: &Map<char>) -> bool {
    let matches_top = frame.check_pattern(&[
        &[EMPTY, EMPTY, SET, EMPTY, EMPTY],
        &[EMPTY, SET, SET, SET, EMPTY],
    ]);
    if matches_top.is_empty() {
        return false;
    }

    let matches_left = frame.check_pattern(&[&[SET, SET], &[EMPTY, SET]]);
    if matches_left.is_empty() {
        return false;
    }

    let matches_right = frame.check_pattern(&[&[SET, SET], &[SET, EMPTY]]);
    if matches_right.is_empty() {
        return false;
    }

    //let mut matches = Vec::new();
    for t in &matches_top {
        for l in &matches_left {
            'brmatches: for r in &matches_right {
                if l.0 <= t.0 || l.1 > t.1 + 1 {
                    continue 'brmatches;
                }
                if r.0 != l.0 || r.1 <= l.1 + 1 {
                    continue 'brmatches;
                }
                if r.1 < t.1 + 3 {
                    continue 'brmatches;
                }
                if r.1 - t.1 - 3 != t.1 - l.1 {
                    continue 'brmatches;
                }
                for jm in t.0 + 2..l.0 {
                    if frame.at((jm, t.1 + 2)) != SET {
                        continue 'brmatches;
                    }
                }
                return true;

                //matches.push((t.0, l.0 + 1, l.1, r.1 + 1));
            }
        }
    }
    false
    /*
    if matches.is_empty() {
        false
    } else {
        for mat in matches {
            println!("match: {mat:?}");
            if let Some(sm) = frame.sub_map((mat.0, mat.1), (mat.2, mat.3)) {
                println!("====");
                sm.print(3);
                println!("====");
            }
        }
        true
    }
    !matches.is_empty()
    */
}

pub fn part2<const HEIGHT: usize, const WIDTH: usize>(input: &str) -> usize {
    let robots = parse_input(input);

    let mut n = 0;
    loop {
        n += 1;

        let frame = get_frame::<HEIGHT, WIDTH>(&robots, n);
        if check_frame_for_christmas_tree(&frame) {
            frame.print(3);
            println!("\nCheck for the christmas tree and rerun starting at {} if this turns out to be a false positive",n+1);

            break;
        }
    }
    n
}
