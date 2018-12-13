use aoc::aoc;

use std::{
    mem,
    fmt::{self, Debug},
};

enum TrackPath {
    Horizontal,
    Vertical,
    Curve(CurveKind),
    Intersection,
    Empty,
}

impl Debug for TrackPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrackPath::Horizontal => f.write_str("-"),
            TrackPath::Vertical => f.write_str("|"),
            TrackPath::Curve(ref c) => {
                match c {
                    CurveKind::BottemLeftToUpRight => f.write_str("/"),
                    CurveKind::UpLeftToBottemRight => f.write_str(r"\\"),
                }
            },
            TrackPath::Intersection => f.write_str("+"),
            TrackPath::Empty => f.write_str(".")
        }
    }
}

pub enum CurveKind {
    BottemLeftToUpRight,
    UpLeftToBottemRight,
}
#[derive(Debug)]
enum TurnState {
    Left,
    Straight,
    Right,
}

impl TurnState {
    fn switch(&mut self) {
        mem::replace(self, match *self {
            TurnState::Left => TurnState::Straight,
            TurnState::Straight => TurnState::Right,
            TurnState::Right => TurnState::Left,
        });
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        mem::replace(self, match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        });
    }

    fn turn_right(&mut self) {
        mem::replace(self, match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        });
    }
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(format!("Tried to create a Direction from {}", c)),
        }
    }
}

#[derive(Debug)]
pub struct Car {
    x: usize,
    y: usize,

    direction: Direction,
    turnstate: TurnState,
}

impl Car {
    pub fn new(x: usize, y: usize, c: char) -> Self {
        let direction = Direction::from(c);
        let turnstate = TurnState::Left;

        Self { x, y, direction, turnstate }
    }

    fn step(&mut self, grid: &[Vec<TrackPath>]) {
        //println!("SELF X Y = {} {}", self.x, self.y);
        //println!("ROW: {:?}", grid[self.y]);
        //println!("TRACKPATH: {:?}", grid[self.y][self.x]);
        match grid[self.y][self.x] {
            TrackPath::Curve(ref c) => {
                match c {
                    CurveKind::UpLeftToBottemRight => {
                        match self.direction {
                            Direction::Up | Direction::Down => self.direction.turn_left(),
                            Direction::Right | Direction::Left => self.direction.turn_right(),
                        }
                    },

                    CurveKind::BottemLeftToUpRight => {
                        match self.direction {
                            Direction::Up | Direction::Down => self.direction.turn_right(),
                            Direction::Right | Direction::Left => self.direction.turn_left(),
                        }
                    }
                }
            }
            TrackPath::Intersection => {
                match self.turnstate {
                    TurnState::Straight => {},
                    TurnState::Left => self.direction.turn_left(),
                    TurnState::Right => self.direction.turn_right(),
                };
                self.turnstate.switch();
            },
            _ => {}
        }

        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };

    }

    fn collide(&self, other: &Car) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<char> for TrackPath {
    fn from(c: char) -> Self {
        match c {
            '-' => TrackPath::Horizontal,
            '|' => TrackPath::Vertical,
            '/' => TrackPath::Curve(CurveKind::BottemLeftToUpRight),
            '\\' => TrackPath::Curve(CurveKind::UpLeftToBottemRight),
            'v' => TrackPath::Vertical,
            '>' => TrackPath::Horizontal,
            '<' => TrackPath::Horizontal,
            '^' => TrackPath::Vertical,
            '+' => TrackPath::Intersection,
            _ => TrackPath::Empty
        }
    }
}
#[aoc(2018, 13, 1)]
fn main(input: &str) {
    
//     let input = r"
// /->-\        
// |   |  /----\
// | /-+--+-\  |
// | | |  | v  |
// \-+-/  \-+--/
//   \------/  
//     ";
    let mut grid = Vec::new();
    let mut cars = Vec::new();

    for (y, line) in input.lines().skip(1).enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            match c {
                '<' | 'v' | '>' | '^' => {
                    let car = Car::new(x, y, c);
                    cars.push(car);
                }
                _ => {}
            }

            row.push(TrackPath::from(c));
        }

        grid.push(row);
    }

    let(x, y) = 'outer: loop {
        for (idx, c1) in cars.iter().enumerate() {
            for c2 in cars[idx + 1..].iter() {
                if c1.collide(c2) {
                    break 'outer (c1.x, c1.y);
                }
            }
        }

        cars.sort_by_key(|car| (car.x, car.y));

        for car in cars.iter_mut() {
            car.step(&grid);
        }
    };

    println!("{} {}", x, y);
}
