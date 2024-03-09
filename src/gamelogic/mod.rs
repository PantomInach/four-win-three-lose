use crate::FIELD_X;
use crate::FIELD_Y;
use crate::{field::Field, GameResult};

pub mod game_handler;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) enum GameState {
    Finished(GameResult),
    InProgress,
}
impl From<&Field> for GameState {
    fn from(field: &Field) -> Self {
        if let Some(player) = field.winner() {
            GameState::Finished(GameResult::from(player))
        } else if let Some(player) = field.loser() {
            GameState::Finished(GameResult::from(player).opposite_player())
        } else if field.set_pieces() == FIELD_X * FIELD_Y {
            GameState::Finished(GameResult::Draw)
        } else {
            GameState::InProgress
        }
    }
}
impl Into<Option<GameResult>> for GameState {
    fn into(self) -> Option<GameResult> {
        match self {
            GameState::Finished(result) => Some(result.clone()),
            GameState::InProgress => None,
        }
    }
}
