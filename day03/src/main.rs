use aoc::aoc;
use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

#[derive(Debug)]
struct ID {
    id: usize,
    from_left: usize,
    from_top: usize,
    width: usize,
    height: usize,
}

lazy_static! {
    static ref ID_REGEX: Regex = Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

impl FromStr for ID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ID_REGEX.captures(s).unwrap();
        Ok(Self {
            id: captures[1].parse().unwrap(),
            from_left: captures[2].parse().unwrap(),
            from_top: captures[3].parse().unwrap(),
            width: captures[4].parse().unwrap(),
            height: captures[5].parse().unwrap(),
        })
    }
}

#[derive(Default)]
pub struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![0; 1000]; 1000],
        }
    }

    fn push_id(&mut self, id: &ID) {
        for row in &mut self.grid[id.from_top..id.from_top + id.height] {
            for cell in &mut row[id.from_left..id.from_left + id.width] {
                *cell += 1;
            }
        }
    }
}

#[aoc(2018, 3, 1)]
fn main(input: &str) -> usize {
    let mut grid = Grid::new();

    for id in input.lines().map(ID::from_str).filter_map(Result::ok) {
        grid.push_id(&id);
    }

    let Grid { grid } = grid;

    grid.into_iter().flatten().filter(|cell| *cell > 1).count()
}
