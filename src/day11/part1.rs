//struct Stone {
//    orig_label: u64,
//    power_of_2024: u64,
//}

//impl Stone {
//    pub fn is_even_digits(&self) {
//        self.orig_label.
//    }
//}

pub fn part1(input: &str) -> usize {
    // go naive at first

    let mut stones = input.to_string();
    for _ in 0..25 {
        let mut new_stones = String::new();

        for label in stones.split(" ").map(|s| s.trim()) {
            let l = label.len();
            if label == "0" {
                new_stones.push('1');
            } else if label.len() % 2 == 0 {
                new_stones.push_str(label[..l / 2].trim_start_matches('0'));
                new_stones.push(' ');
                new_stones.push_str({
                    let tail = label[l / 2..].trim_start_matches('0');
                    if tail.is_empty() {
                        "0"
                    } else {
                        tail
                    }
                });
            } else {
                new_stones.push_str((label.parse::<u64>().unwrap() * 2024).to_string().trim());
            }
            new_stones.push(' ');
        }
        stones = new_stones.trim().to_owned();
    }

    stones.split(" ").count()
}
