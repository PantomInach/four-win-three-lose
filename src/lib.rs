use std::fmt::Display;

use field::GameResult;

mod brute_force_field;
pub mod field;
pub mod gamelogic;
pub mod player;
pub mod visualizer;

// Changing these values will break the current implementations of [winner] and [loser].
pub const FIELD_X: usize = 4;
pub const FIELD_Y: usize = 4;

pub(crate) type NextBestMove = (Position, GameResult);
/// Position contains (x-coordinate, y-coordinate).
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}
impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {
            x: value.0,
            y: value.1,
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
