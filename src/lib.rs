use std::fmt::Display;

mod brute_force_field;
pub mod field;
pub mod gamelogic;
pub mod player;
pub mod visualizer;

// Changing these values will break the current implementations of [winner] and [loser].
pub const FIELD_X: usize = 4;
pub const FIELD_Y: usize = 4;

pub(crate) type NextBestMove = (Position, GameResult, usize);
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

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum GameResult {
    Draw,
    PlayerOneWins,
    PlayerTwoWins,
}
impl From<bool> for GameResult {
    fn from(value: bool) -> Self {
        match value {
            true => GameResult::PlayerTwoWins,
            false => GameResult::PlayerOneWins,
        }
    }
}
impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            GameResult::Draw => "Draw! Nobody wins.",
            GameResult::PlayerOneWins => "Player One wins!",
            GameResult::PlayerTwoWins => "Player Two wins!",
        };
        write!(f, "{}", message)
    }
}
impl GameResult {
    #[allow(dead_code)]
    fn player_to_result(player: Option<bool>) -> GameResult {
        match player {
            None => GameResult::Draw,
            Some(false) => GameResult::PlayerOneWins,
            Some(true) => GameResult::PlayerTwoWins,
        }
    }

    pub(crate) fn opposite_player(&self) -> GameResult {
        match self {
            GameResult::Draw => GameResult::Draw,
            GameResult::PlayerOneWins => GameResult::PlayerTwoWins,
            GameResult::PlayerTwoWins => GameResult::PlayerOneWins,
        }
    }

    pub(crate) fn better_eq_for_player(&self, other: &GameResult, evaluate_for: bool) -> bool {
        #[allow(clippy::if_same_then_else)]
        if self == other {
            true
        } else if self == &GameResult::from(evaluate_for) {
            true
        } else {
            other == &GameResult::from(evaluate_for).opposite_player()
        }
    }
}
