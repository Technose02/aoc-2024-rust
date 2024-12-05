pub fn part2(input: &str) -> usize {
    let rows = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let row_count = rows.len();
    let col_count = rows[0].len();

    let mut columns = Vec::new();

    for _j in 0..col_count {
        columns.push(Vec::new());
    }
    for row in &rows {
        for (i, c) in row.iter().enumerate() {
            columns[i].push(*c);
        }
    }

    let mut s = 0;

    // check blocks
    for j in 0..=row_count - 3 {
        for i in 0..=col_count - 3 {
            let middle = rows[j + 1][i + 1];
            let ul = rows[j][i];
            let ur = rows[j][i + 2];
            let ll = rows[j + 2][i];
            let lr = rows[j + 2][i + 2];

            if middle == 'A' {
                match (ul, lr, ll, ur) {
                    ('M', 'S', 'M', 'S')
                    | ('M', 'S', 'S', 'M')
                    | ('S', 'M', 'M', 'S')
                    | ('S', 'M', 'S', 'M') => {
                        s += 1;
                    }
                    _ => (),
                }
            }
        }
    }

    s
}
