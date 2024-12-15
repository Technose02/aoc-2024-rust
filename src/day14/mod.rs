use crate::util::extended_modulo;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[derive(Debug, Clone, Copy)]
struct RobotPosition {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct RobotVelocity {
    v_x: isize,
    v_y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: RobotPosition,
    velocity: RobotVelocity,
}

impl Robot {
    pub fn predict_future_position(
        &self,
        seconds: usize,
        area_height: usize,
        area_width: usize,
    ) -> RobotPosition {
        let x = self.position.x as isize + seconds as isize * self.velocity.v_x;

        let y = self.position.y as isize + seconds as isize * self.velocity.v_y;
        RobotPosition {
            x: extended_modulo(x, area_width),
            y: extended_modulo(y, area_height),
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let mut robot_spec = line
            .split(" ")
            .map(|p| p.trim().split("=").nth(1).unwrap())
            .flat_map(|p| p.trim().split(","))
            .map(|n| n.trim().parse::<isize>().unwrap());

        robots.push(Robot {
            position: RobotPosition {
                x: robot_spec.next().unwrap() as usize,
                y: robot_spec.next().unwrap() as usize,
            },
            velocity: RobotVelocity {
                v_x: robot_spec.next().unwrap(),
                v_y: robot_spec.next().unwrap(),
            },
        });
    }

    robots
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    const PART1_OUTPUT: usize = 12;

    #[test]
    fn day14_part1_works() {
        assert_eq!(part1::<7, 11>(TEST_INPUT), PART1_OUTPUT);
    }
}
