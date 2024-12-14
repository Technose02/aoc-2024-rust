use std::ops::Add;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;
const MAX_BUTTON_PUSHES: usize = 100;

#[derive(Debug)]
pub struct ClawButton {
    move_x: usize,
    move_y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClawPosition {
    x: usize,
    y: usize,
}

impl Add<usize> for ClawPosition {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        ClawPosition {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

#[derive(Debug)]
pub struct ClawGame {
    button_a: ClawButton,
    button_b: ClawButton,
    prize_pos: ClawPosition,
}

impl ClawGame {
    pub fn apply_pushes(&self, button_a_pushes: usize, button_b_pushes: usize) -> ClawPosition {
        ClawPosition {
            x: self.button_a.move_x * button_a_pushes + self.button_b.move_x * button_b_pushes,
            y: self.button_a.move_y * button_a_pushes + self.button_b.move_y * button_b_pushes,
        }
    }
}

pub struct Lgs2 {
    pub a_11: f64,
    pub a_12: f64,
    pub a_21: f64,
    pub a_22: f64,
    pub b_1: f64,
    pub b_2: f64,
}

impl Lgs2 {
    pub fn from_game(game: &ClawGame) -> Self {
        let ClawGame {
            button_a:
                ClawButton {
                    move_x: a_x,
                    move_y: a_y,
                },
            button_b:
                ClawButton {
                    move_x: b_x,
                    move_y: b_y,
                },
            prize_pos: ClawPosition { x: p_x, y: p_y },
        } = game;

        let (a_x_f, a_y_f, b_x_f, b_y_f, p_x_f, p_y_f) = (
            *a_x as f64,
            *a_y as f64,
            *b_x as f64,
            *b_y as f64,
            *p_x as f64,
            *p_y as f64,
        );

        Lgs2 {
            a_11: a_x_f,
            a_12: b_x_f,
            a_21: a_y_f,
            a_22: b_y_f,
            b_1: p_x_f,
            b_2: p_y_f,
        }
    }

    pub fn solve<S>(&self, solver: S) -> Option<(f64, f64)>
    where
        S: Fn(&Lgs2) -> Option<(f64, f64)>,
    {
        solver(self)
    }
}

fn parse_input(input: &str) -> Vec<ClawGame> {
    let mut lines = input.lines();
    let mut games = Vec::new();
    while let Some(l) = lines.next() {
        let mut la = l
            .strip_prefix("Button A:")
            .expect("wrong line format: expected line to start with 'Button A:'")
            .split(",")
            .map(|s| {
                s.trim()[2..].parse::<usize>().expect(
                    "wrong input format: expected move values for Button A to be parseble as usize",
                )
            });

        let mut lb = lines.next().expect("wrong input format: expected definition for Button A to be followed by definition of Button B")
            .strip_prefix("Button B: ")
            .expect("wrong line format: expected line to start with 'Button B: '")
            .split(",")
            .map(|s| s.trim()[2..].parse::<usize>().expect("wrong input format: expected move values for Button B to be parseble as usize"));

        let mut lp = lines.next().expect("wrong input format: expected definition for Button B to be followed by definition of Prize-Position")
            .strip_prefix("Prize: ")
            .expect("wrong line format: expected line to start with 'Prize: '")
            .split(",")
            .map(|s| s.trim()[2..].parse::<usize>().expect("wrong input format: expected positional values for Prize to be parseble as usize"));

        let button_a = ClawButton {
            move_x: la.next().expect(
                "wrong input format: expected definition for Button A to have a Move-X-Component",
            ),
            move_y: la.next().expect(
                "wrong input format: expected definition for Button A to have a Move-Y-Component",
            ),
        };
        let button_b = ClawButton {
            move_x: lb.next().expect(
                "wrong input format: expected definition for Button B to have a Move-X-Component",
            ),
            move_y: lb.next().expect(
                "wrong input format: expected definition for Button B to have a Move-Y-Component",
            ),
        };
        let prize_pos = ClawPosition {
            x: lp.next().expect(
                "wrong input format: expected definition for Prize-Position to have a X-Component",
            ),
            y: lp.next().expect(
                "wrong input format: expected definition for Prize-Position to have a Y-Component",
            ),
        };
        games.push(ClawGame {
            button_a,
            button_b,
            prize_pos,
        });

        _ = lines.next(); // read empty line if more games present
    }
    games
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    const PART1_OUTPUT: usize = 480;

    #[test]
    fn day13_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 875318608908;

    #[test]
    fn day13_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
