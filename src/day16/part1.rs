use std::collections::{HashMap, HashSet};

struct Maze {
    walls: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_maze(input: &str) -> Maze {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    continue;
                }
                '#' => {
                    walls.insert((j, i));
                }
                'S' => {
                    start = Some((j, i));
                }
                'E' => {
                    end = Some((j, i));
                }
                _ => panic!("unexpected char '{c}' encountered while parsing"),
            }
        }
    }

    Maze {
        walls,
        start: start.expect("err: no start found in maze-spec"),
        end: end.expect("err: no end found in maze-spec"),
    }
}

type MazeTraverseJob = ((usize, usize), usize, Orientation);

struct MazeTraverse<'a> {
    visited: HashMap<(usize, usize), usize>,
    maze: &'a Maze,
    jobs: Vec<MazeTraverseJob>,
}

impl<'a> MazeTraverse<'a> {
    fn new(maze: &'a Maze) -> Self {
        let mut jobs = Vec::<MazeTraverseJob>::with_capacity(1800);
        jobs.push((maze.start, 0, Orientation::East));
        MazeTraverse {
            visited: HashMap::new(),
            maze,
            jobs,
        }
    }

    fn run(&mut self) -> usize {
        while let Some((pos, score, orientation)) = self.jobs.pop() {
            if self.maze.walls.contains(&pos) {
                continue;
            }

            if let Some(registered_score) = self.visited.get(&pos) {
                if *registered_score < score {
                    continue;
                }
            }

            if let Some(registered_score) = self.visited.get_mut(&pos) {
                *registered_score = score;
            } else {
                self.visited.insert(pos, score);
            }

            if pos != self.maze.end {
                let orientation_90c = orientation.rotate();
                let orientation_90cc = orientation_90c.rotate().rotate();

                self.jobs
                    .push((orientation.one_step(pos), score + 1, orientation));
                self.jobs
                    .push((orientation_90c.one_step(pos), score + 1001, orientation_90c));
                self.jobs.push((
                    orientation_90cc.one_step(pos),
                    score + 1001,
                    orientation_90cc,
                ));
            }
        }

        *self
            .visited
            .iter()
            .find(|(&k, _)| k == self.maze.end)
            .expect("no path from S to E could be found!")
            .1
    }
}

enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn rotate(&self) -> Self {
        match self {
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::North => Orientation::East,
        }
    }
    fn one_step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Orientation::East => (pos.0, pos.1 + 1),
            Orientation::South => (pos.0 + 1, pos.1),
            Orientation::West => (pos.0, pos.1 - 1),
            Orientation::North => (pos.0 - 1, pos.1),
        }
    }
}

impl Maze {
    pub fn traverse(&self) -> usize {
        MazeTraverse::new(self).run()
    }
}

pub fn part1(input: &str) -> usize {
    parse_maze(input).traverse()
}
