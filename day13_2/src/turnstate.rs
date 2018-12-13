use std::mem;

/// Represents the state of what direction should be moved in upon an intersection point.
#[derive(Debug)]
pub enum TurnState {
    Left,
    Straight,
    Right,
}

impl TurnState {
    pub fn new() -> Self {
        TurnState::Left
    }

    pub fn switch(&mut self) {
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
