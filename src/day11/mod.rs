use std::collections::HashMap;

mod part1;
mod part2;

pub use part1::part1;
pub use part2::part2;

#[derive(Debug)]
struct StoneCounts(HashMap<usize, usize>);

impl StoneCounts {
    pub fn new(labels: Vec<usize>) -> Self {
        let mut stone_counts = StoneCounts(HashMap::new());
        for l in labels {
            stone_counts.increase_count(l, 1);
        }

        stone_counts
    }

    fn increase_count(&mut self, label: usize, amount: usize) {
        if let Some(v) = self.0.get_mut(&label) {
            *v += amount;
        } else {
            self.0.insert(label, amount);
        }
    }

    fn decrease_count(&mut self, label: usize, amount: usize) {
        let current_count = self
            .0
            .get_mut(&label)
            .expect("only present labels can be decreased in count!");
        if *current_count < amount {
            panic!("cannot remove more stones with label '{label}' than are present")
        }
        *current_count -= amount;
    }

    pub fn blink(&mut self) {
        let labels_with_counts: Vec<(usize, usize)> = self
            .0
            .iter()
            .filter(|&t| *t.1 > 0)
            .map(|t| (*t.0, *t.1))
            .collect();

        labels_with_counts.iter().for_each(|&(label, count)| {
            self.decrease_count(label, count);

            if label == 0 {
                self.increase_count(1, count);
            } else {
                let str_label = label.to_string();
                let l = str_label.len();
                if l % 2 == 0 {
                    let label_upper = str_label[..l / 2].parse::<usize>().unwrap();
                    let label_lower = str_label[l / 2..].parse::<usize>().unwrap();
                    self.increase_count(label_upper, count);
                    self.increase_count(label_lower, count);
                } else {
                    self.increase_count(label * 2024, count);
                }
            }
        });
    }
    pub fn count(&self) -> usize {
        self.0.iter().map(|(_, &v)| v).sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = r#"125 17"#;

    const PART1_OUTPUT: usize = 55312;

    #[test]
    fn day11_part1_works() {
        assert_eq!(part1(TEST_INPUT), PART1_OUTPUT);
    }

    const PART2_OUTPUT: usize = 65601038650482;

    #[test]
    fn day11_part2_works() {
        assert_eq!(part2(TEST_INPUT), PART2_OUTPUT);
    }
}
