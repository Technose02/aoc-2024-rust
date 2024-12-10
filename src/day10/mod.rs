mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

pub struct Map {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl Map {
    pub fn parse(input: &str) -> Map {
        let mut lines = input.lines();
        let width = lines.next().unwrap().chars().count();
        let height = lines.count() + 1;

        let mut data = Vec::<Vec<usize>>::with_capacity(height);
        for line in input.lines() {
            data.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>(),
            );
        }

        Map {
            data,
            width,
            height,
        }
    }

    pub fn at(&self, pos: (usize, usize)) -> usize {
        self.data[pos.0][pos.1]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> impl Iterator<Item = &[usize]> {
        self.data.iter().map(|r| r.as_slice())
    }

    pub fn print(&self, indent: usize) {
        for row in self.rows() {
            print!("{}", " ".repeat(indent));
            println!("{}", row.iter().map(|d| d.to_string()).collect::<String>())
        }
    }
    pub fn potential_trails(&self) -> Vec<(usize, usize)> {
        let mut potential_trail_heads = Vec::new();
        for (j, row) in self.rows().enumerate() {
            for (i, &val) in row.iter().enumerate() {
                if val == 0 {
                    potential_trail_heads.push((j, i));
                }
            }
        }
        potential_trail_heads
    }

    pub fn top(&self, pos: (usize, usize)) -> Option<usize> {
        if pos.0 > 0 {
            Some(self.at((pos.0 - 1, pos.1)))
        } else {
            None
        }
    }

    pub fn bottom(&self, pos: (usize, usize)) -> Option<usize> {
        if pos.0 < self.height - 1 {
            Some(self.at((pos.0 + 1, pos.1)))
        } else {
            None
        }
    }

    pub fn left(&self, pos: (usize, usize)) -> Option<usize> {
        if pos.1 > 0 {
            Some(self.at((pos.0, pos.1 - 1)))
        } else {
            None
        }
    }

    pub fn right(&self, pos: (usize, usize)) -> Option<usize> {
        if pos.1 < self.width - 1 {
            Some(self.at((pos.0, pos.1 + 1)))
        } else {
            None
        }
    }

    pub fn check_subtrail_start(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let val = self.at(start);

        if val == 9 {
            res.push(start);
            return res;
        }

        if let Some(left_val) = self.left(start) {
            if left_val == val + 1 {
                res.extend(self.check_subtrail_start((start.0, start.1 - 1)));
            }
        }

        if let Some(right_val) = self.right(start) {
            if right_val == val + 1 {
                res.extend(&self.check_subtrail_start((start.0, start.1 + 1)));
            }
        }

        if let Some(top_val) = self.top(start) {
            if top_val == val + 1 {
                res.extend(&self.check_subtrail_start((start.0 - 1, start.1)));
            }
        }

        if let Some(bottom_val) = self.bottom(start) {
            if bottom_val == val + 1 {
                res.extend(&self.check_subtrail_start((start.0 + 1, start.1)));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    const PART1_OUTPUT: usize = 36;

    #[test]
    fn day10_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 81;

    #[test]
    fn day10_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
