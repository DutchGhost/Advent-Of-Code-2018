use aoc::aoc;

mod direction;
mod trackpath;
mod turnstate;

use self::direction::Direction;
use self::trackpath::{CurveKind, TrackPath};
use self::turnstate::TurnState;

#[derive(Debug)]
pub struct Cart {
    id: u32,
    x: u32,
    y: u32,

    direction: Direction,
    turnstate: TurnState,
}

impl Cart {
    pub fn new(id: u32, x: u32, y: u32, c: char) -> Self {
        let direction = Direction::from(c);
        let turnstate = TurnState::new();

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

        match grid[self.y as usize][self.x as usize] {
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

    fn collide(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y && self.id != other.id
    }
}

pub fn parse_track(s: &str) -> (Vec<Vec<TrackPath>>, Vec<Cart>) {
    let mut track = Vec::new();
    let mut carts = Vec::new();

    let mut cart_id = 0;

    for (y, line) in s.lines().enumerate() {
        let mut row = Vec::new();
        for (x, trackpath) in line.chars().enumerate() {
            match trackpath {
                '<' | 'v' | '>' | '^' => {
                    let cart = Cart::new(cart_id, x as u32, y as u32, trackpath);
                    carts.push(cart);
                    cart_id += 1;
                }
                _ => {}
            }
            row.push(TrackPath::from(trackpath));
        }
        track.push(row);
    }

    (track, carts)
}

#[aoc(2018, 13, 2)]
fn main(input: &str) -> (u32, u32) {
    let (track, mut carts) = parse_track(input);

    let mut collided_carts = Vec::new();

    while carts.len() != 1 {
        carts.sort_by_key(|cart| (cart.y, cart.x));

        for idx in 0..carts.len() {
            carts[idx].step(&track);

            for cart in &carts {
                if cart.collide(&carts[idx]) {
                    collided_carts.push(cart.id);
                    collided_carts.push(carts[idx].id);
                }
            }
        }

        carts.retain(|cart| !collided_carts.contains(&cart.id));
        collided_carts.clear();
    }

    let remaining_cart = &carts[0];

    (remaining_cart.x, remaining_cart.y)
}
