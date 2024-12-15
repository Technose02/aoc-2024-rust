pub struct Map<T>
where
    T: Copy + From<char> + Into<String>,
{
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T>
where
    T: Copy + PartialEq + From<char> + Into<String>,
{
    pub fn new(height: usize, width: usize, default: T) -> Self {
        let data = vec![vec![default; width]; height];
        Map {
            data,
            width,
            height,
        }
    }

    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let width = lines.next().unwrap().chars().count();
        let height = lines.count() + 1;

        let mut data = Vec::<Vec<T>>::with_capacity(height);
        for line in input.lines() {
            data.push(line.chars().map(|c| c.into()).collect::<Vec<T>>());
        }

        Map {
            data,
            width,
            height,
        }
    }

    pub fn at(&self, pos: (usize, usize)) -> T {
        self.data[pos.0][pos.1]
    }

    pub fn set_at(&mut self, pos: (usize, usize), val: T) {
        self.data[pos.0][pos.1] = val;
    }

    pub fn check_pattern(&self, pattern: &[&[T]]) -> Vec<(usize, usize)> {
        let p_height = pattern.len();
        if p_height == 0 || p_height > self.height {
            return Vec::new();
        }
        let p_width = pattern[0].len();
        if p_width == 0 || p_width > self.width {
            return Vec::new();
        }

        let mut res = Vec::new();
        for j in 0..self.height - p_height {
            'width: for i in 0..self.width - p_width {
                // check map for match with pattern at (j,i)
                for (pj, p_row) in pattern.iter().enumerate() {
                    for pi in 0..p_width {
                        if self.at((j + pj, i + pi)) != p_row[pi] {
                            continue 'width;
                        }
                    }
                }
                res.push((j, i));
            }
        }

        res
    }

    pub fn sub_map(&self, j_range: (usize, usize), i_range: (usize, usize)) -> Option<Self> {
        if j_range.1 <= j_range.0
            || i_range.1 <= i_range.0
            || j_range.1 >= self.height
            || i_range.1 >= self.width
        {
            return None;
        }

        let sub_height = j_range.1 - j_range.0 + 1;
        let sub_width = i_range.1 - i_range.0 + 1;
        let mut sub_data = Vec::with_capacity(sub_height);
        for row in &self.data[j_range.0..=j_range.1] {
            let mut sub_row = Vec::with_capacity(sub_width);
            for col in &row[i_range.0..=i_range.1] {
                sub_row.push(*col);
            }
            sub_data.push(sub_row);
        }

        Some(Map {
            data: sub_data,
            height: sub_height,
            width: i_range.1 - i_range.0 + 1,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|r| r.as_slice())
    }

    #[allow(unused)]
    pub fn print(&self, indent: usize) {
        for row in self.rows() {
            print!("{}", " ".repeat(indent));
            println!("{}", row.iter().map(|&d| d.into()).collect::<String>())
        }
    }
}
