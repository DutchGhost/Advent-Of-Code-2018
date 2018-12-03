#![feature(never_type)]

use std::str::FromStr;

const PUZZLE: &str = include_str!("input.txt");

#[derive(Debug)]
struct ID {
    id: usize,
    from_left: usize,
    from_top: usize,
    width: usize,
    height: usize,
}

impl FromStr for ID {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut yank_id = s.split(" @");
        let mut id_part = yank_id.next().expect("Failed to parse id part");
        id_part = &id_part[1..];

        let rest = yank_id.next().expect("Failed to parse the rest");

        let mut yank_from_left = rest.split(",");
        let from_left = yank_from_left
            .next()
            .expect("Failed to parse from_left part");

        let rest = yank_from_left.next().expect("Failed to parse the rest");

        let mut yank_from_top = rest.split(": ");
        let from_top = yank_from_top.next().expect("Failed to parse from_top");

        let rest = yank_from_top.next().expect("Failed to parse the rest");

        let mut yank_width = rest.split("x");
        let width = yank_width.next().expect("Failed to parse width");
        let height = yank_width.next().expect("Failed to parse height");

        Ok(Self {
            id: id_part.trim().parse().unwrap(),
            from_left: from_left.trim().parse().unwrap(),
            from_top: from_top.parse().unwrap(),
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
        })
    }
}

struct Grid {
    height: usize,
    width: usize,
    grid: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            height: 0,
            width: 0,
            grid: Vec::new(),
        }
    }
    fn width_extend_to_fit(&mut self, id: &ID) {
        let id_size = id.from_left + id.width;
        if self.width < id_size {
            let to_grow = id_size - self.width;

            for row in self.grid.iter_mut() {
                row.extend((0..to_grow).map(|_| 0));
            }

            self.width += to_grow;
        }
    }

    fn height_extend_to_fit(&mut self, id: &ID) {
        let id_height = id.from_top + id.height;

        if self.height < id_height {
            let to_grow = id_height - self.height;
            let width = self.width;

            self.grid.extend((0..to_grow).map(|_| vec![0; width]));

            self.height += to_grow;
        }
    }
    fn push_id(&mut self, id: ID) {
        self.width_extend_to_fit(&id);
        self.height_extend_to_fit(&id);

        for row in &mut self.grid[id.from_top..id.from_top + id.height].iter_mut() {
            for cell in &mut row[id.from_left..id.from_left + id.width] {
                *cell += 1;
            }
        }
    }
}
fn main() {
    let mut grid = Grid::new();

    for line in PUZZLE.lines() {
        grid.push_id(line.parse::<ID>().unwrap());
    }

    let total = grid
        .grid
        .into_iter()
        .flatten()
        .filter(|cell| *cell > 1)
        .count();

    println!("{}", total);
}
