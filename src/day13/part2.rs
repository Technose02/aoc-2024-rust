use super::{parse_input, ClawGame, Lgs2, BUTTON_A_COST, BUTTON_B_COST};

const FBN: usize = 10000000000000;

fn min_cost_for_game_part2(game: &ClawGame) -> Option<usize> {
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
        //   (           ) (   ) = (   ) + FBN
        //   ( a_21 a_22 ) (x_2)   (b_2)
        //
        //   with FBN (Fu**in' big number) = 10000000000000

        let alpha = a_11 / a_21;
        let fbn_f: f64 = FBN as f64;

        // here we explicitly avoid 'catastrophic cancellation' which might be potentially introduced in the rather intuitive term `(b_1 + FBN) - (b_2 + FBN) * alpha`
        let x_2 = (b_1 - b_2 * alpha + fbn_f * (1_f64 - alpha)) / (a_12 - a_22 * alpha);
        let x_1 = (b_2 + fbn_f - a_22 * x_2) / a_21;

        Some((x_1, x_2))
    };

    if let Some((a_raw, b_raw)) = Lgs2::from_game(game).solve(solver) {
        let b = b_raw.round() as usize;
        let a = a_raw.round() as usize;

        if game.apply_pushes(a, b) != game.prize_pos + FBN {
            //println!("equations solved but rounded result is wrong, so it just is not solveable for whole numbers (raw result: ({a_raw},{b_raw}))");
            return None;
        }

        //println!("solved: push Button 'A' {a} time(s) and Button 'B' {b} time(s)");
        Some(BUTTON_A_COST * a + BUTTON_B_COST * b)
    } else {
        None
    }
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .flat_map(&min_cost_for_game_part2)
        .sum()
}
