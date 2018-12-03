#![feature(never_type)]

use lazy_static::lazy_static;
use regex::Regex;

use std::{collections::HashSet, str::FromStr};

const PUZZLE: &str = include_str!("input.txt");

#[derive(Debug, Hash)]
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
    type Err = !;

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

#[derive(Debug, Clone, Default)]
struct Entry {
    id: usize,
    value: usize,
}

pub struct Grid {
    /// Represents the grid. This is 1000 by 1000.
    grid: Vec<Vec<Entry>>,

    /// Represents the id's that overlapped.
    overlapped: HashSet<usize>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![Entry::default(); 1000]; 1000],
            overlapped: HashSet::new(),
        }
    }

    fn push_id(&mut self, id: ID) {
        for row in &mut self.grid[id.from_top..id.from_top + id.height] {
            for cell in &mut row[id.from_left..id.from_left + id.width] {
                if cell.value == 0 {
                    cell.value = 1;
                    cell.id = id.id;
                } else {
                    self.overlapped.insert(cell.id);
                    self.overlapped.insert(id.id);
                }
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new();

    for id in PUZZLE.lines().map(ID::from_str).filter_map(Result::ok) {
        grid.push_id(id);
    }

    let Grid { grid, overlapped } = grid;

    let not_overlappedd = grid
        .into_iter()
        .flatten()
        .filter(|cell| cell.id != 0)
        .find(|cell| !overlapped.contains(&cell.id));

    println!("{:?}", not_overlappedd);
}
