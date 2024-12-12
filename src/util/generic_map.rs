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
    T: Copy + From<char> + Into<String>,
{
    pub fn parse(input: &str) -> Map<T> {
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
