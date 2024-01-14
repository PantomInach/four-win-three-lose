use field::GameResult;

pub mod field;
mod player;
mod visualizer;

// Changing these values will break the current implementations of [winner] and [loser].
pub const FIELD_X: usize = 4;
pub const FIELD_Y: usize = 4;

/// Position contains (x-coordinate, y-coordinate).
pub(crate) type Position = (usize, usize);
pub(crate) type NextBestMove = (Position, GameResult);
