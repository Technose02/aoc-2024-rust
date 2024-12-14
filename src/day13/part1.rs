use super::{parse_input, ClawGame, Lgs2, BUTTON_A_COST, BUTTON_B_COST, MAX_BUTTON_PUSHES};

fn min_cost_for_game_part1(game: &ClawGame) -> Option<usize> {
    let solver = |lgs2: &Lgs2| {
        let Lgs2 {
            a_11,
            a_12,
            a_21,
            a_22,
            b_1,
            b_2,
        } = *lgs2;

        // in part1 we just need to solve this system of 2 linear equations:
        //
        //   ( a_11 a_12 ) (x_1)   (b_1)
        //   (           ) (   ) = (   )
        //   ( a_21 a_22 ) (x_2)   (b_2)

        let alpha = a_11 / a_21;
        let x_2 = (b_1 - b_2 * alpha) / (a_12 - a_22 * alpha);
        let x_1 = (b_2 - a_22 * x_2) / a_21;

        Some((x_1, x_2))
    };

    if let Some((a_raw, b_raw)) = Lgs2::from_game(game).solve(solver) {
        let b = b_raw.round() as usize;
        if b > MAX_BUTTON_PUSHES {
            //println!(
            //    "no prizes unless you push button 'B' more than {MAX_BUTTON_PUSHES} ({b}!!!) time(s)"
            //);
            return None;
        }

        let a = a_raw.round() as usize;
        if a > MAX_BUTTON_PUSHES {
            //println!(
            //    "no prizes unless you push button 'A' more than {MAX_BUTTON_PUSHES} ({a}!!!) time(s)"
            //);
            return None;
        }

        if game.apply_pushes(a, b) != game.prize_pos {
            //println!("equation solved but rounded result is wrong, so it just is not solveable for whole numbers (raw result: ({a_raw},{b_raw}))");
            return None;
        }

        //println!("solved: push Button 'A' {a} time(s) and Button 'B' {b} time(s)");
        Some(BUTTON_A_COST * a + BUTTON_B_COST * b)
    } else {
        None
    }
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .flat_map(&min_cost_for_game_part1)
        .sum()
}
