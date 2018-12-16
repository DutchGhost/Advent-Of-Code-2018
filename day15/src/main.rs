use aoc::aoc;

#[derive(Debug)]
pub struct Elf {
    x: usize,
    y: usize,
    health: usize,
}

impl Elf {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, health: 200 }
    }
}

#[derive(Debug)]
pub struct Goblin {
    x: usize,
    y: usize,
    health: usize,
}

impl Goblin {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, health: 200 }
    }
}

#[derive(Debug)]
pub enum Unit {
    Elf(Elf),
    Goblin(Goblin),
}

#[derive(Debug)]
pub enum Node {
    Wall,
    Cavern,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            '#' => Node::Wall,
            '.' => Node::Cavern,
            _ => Node::Cavern,
        }
    }
}

pub type Area = Vec<Vec<Node>>;

pub fn parse_area(s: &str) -> (Area, Vec<Unit>) {
    let area = s
        .lines()
        .map(|row| row.chars().map(|cell| Node::from(cell)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let units = s
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(x, cell)| *cell == 'G' || *cell == 'E')
                .map(move |(x, cell)| match cell {
                    'G' => Unit::Goblin(Goblin::new(x, y)),
                    _ => Unit::Elf(Elf::new(x, y)),
                })
        }).collect::<Vec<_>>();

    (area, units)
}

#[aoc(2018, 15, 1)]
fn main(input: &str) {
    let (area, units) = parse_area(input);

    for row in area {
        for cell in row {
            match cell {
                Node::Wall => print!("#"),
                Node::Cavern => print!("."),
            }
        }
        println!()
    }
}
