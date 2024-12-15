use super::parse_input;

pub fn part1<const HEIGHT: usize, const WIDTH: usize>(input: &str) -> usize {
    let robots = parse_input(input);

    let (mut sq1, mut sq2, mut sq3, mut sq4) = (0, 0, 0, 0);

    for robot in robots {
        let p = robot.predict_future_position(100, HEIGHT, WIDTH);
        if p.y < HEIGHT / 2 && p.x < WIDTH / 2 {
            sq1 += 1;
            //println!("({},{}) -> Q1", p.y, p.x);
        } else if p.y < HEIGHT / 2 && p.x > WIDTH / 2 {
            sq2 += 1;
            //println!("({},{}) -> Q2", p.y, p.x);
        } else if p.y > HEIGHT / 2 && p.x > WIDTH / 2 {
            sq3 += 1;
            //println!("({},{}) -> Q3", p.y, p.x);
        } else if p.y > HEIGHT / 2 && p.x < WIDTH / 2 {
            sq4 += 1;
            //println!("({},{}) -> Q4", p.y, p.x);
        }
    }

    sq1 * sq2 * sq3 * sq4
}
