fn count_xmasses(chars: &[char]) -> usize {
    let chars = chars.iter().collect::<String>();
    if chars == "XMAS" || chars == "SAMX" {
        1
    } else {
        0
    }
}

pub fn part1(input: &str) -> usize {
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

    // check all horizontal possibilities
    for row in &rows {
        for i in 0..=row_count - 4 {
            s += count_xmasses(&row[i..i + 4]);
        }
    }

    // check all vertical possibilities
    for col in &columns {
        for j in 0..=col_count - 4 {
            s += count_xmasses(&col[j..j + 4]);
        }
    }

    // check diagonals
    for j in 0..=row_count - 4 {
        for i in 0..=col_count - 4 {
            s += count_xmasses(&[
                rows[j][i],
                rows[j + 1][i + 1],
                rows[j + 2][i + 2],
                rows[j + 3][i + 3],
            ] as &[char]);

            s += count_xmasses(&[
                rows[j + 3][i],
                rows[j + 2][i + 1],
                rows[j + 1][i + 2],
                rows[j][i + 3],
            ] as &[char]);
        }
    }

    s
}
