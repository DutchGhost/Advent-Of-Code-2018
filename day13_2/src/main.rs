use aoc::aoc;

use std::{
    fmt::{self, Debug},
    mem,
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
            TrackPath::Curve(ref c) => match c {
                CurveKind::BottemLeftToUpRight => f.write_str("c"),
                CurveKind::UpLeftToBottemRight => f.write_str("c"),
            },
            TrackPath::Intersection => f.write_str("+"),
            TrackPath::Empty => f.write_str("."),
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
        mem::replace(
            self,
            match *self {
                TurnState::Left => TurnState::Straight,
                TurnState::Straight => TurnState::Right,
                TurnState::Right => TurnState::Left,
            },
        );
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
        mem::replace(
            self,
            match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
        );
    }

    fn turn_right(&mut self) {
        mem::replace(
            self,
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        );
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

    id: usize,
}

impl Car {
    pub fn new(id: usize, x: usize, y: usize, c: char) -> Self {
        let direction = Direction::from(c);
        let turnstate = TurnState::Left;

        Self {
            id,
            x,
            y,
            direction,
            turnstate,
        }
    }

    fn step(&mut self, grid: &[Vec<TrackPath>]) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };

        match grid[self.y][self.x] {
            TrackPath::Curve(ref c) => match c {
                CurveKind::UpLeftToBottemRight => match self.direction {
                    Direction::Up | Direction::Down => self.direction.turn_left(),
                    Direction::Right | Direction::Left => self.direction.turn_right(),
                },

                CurveKind::BottemLeftToUpRight => match self.direction {
                    Direction::Up | Direction::Down => self.direction.turn_right(),
                    Direction::Right | Direction::Left => self.direction.turn_left(),
                },
            },
            TrackPath::Intersection => {
                match self.turnstate {
                    TurnState::Straight => {}
                    TurnState::Left => self.direction.turn_left(),
                    TurnState::Right => self.direction.turn_right(),
                };
                self.turnstate.switch();
            }
            _ => {}
        }
    }

    fn collide(&self, other: &Car) -> bool {
        self.x == other.x && self.y == other.y && self.id != other.id
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
            _ => TrackPath::Empty,
        }
    }
}
#[aoc(2018, 13, 1)]
fn main(input: &str) -> (usize, usize) {
    let mut grid = Vec::new();
    let mut cars = Vec::new();
    let mut id = 0;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            match c {
                '<' | 'v' | '>' | '^' => {
                    let car = Car::new(id, x, y, c);
                    cars.push(car);
                    id += 1;
                }
                _ => {}
            }

            row.push(TrackPath::from(c));
        }
        grid.push(row);
    }

    while cars.len() > 1 {
        cars.sort_by_key(|car| (car.y, car.x));

        let mut collided = Vec::new();

        for car_idx in 0..cars.len() {
            cars[car_idx].step(&grid);

            for c1 in cars.iter() {
                if c1.collide(&cars[car_idx]) {
                    collided.push(c1.id);
                    collided.push(cars[car_idx].id);
                }
            }
        }

        cars.retain(|c| !collided.contains(&c.id));
    }

    (cars[0].x, cars[0].y)
}
