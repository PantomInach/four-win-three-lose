use crate::field::{Field, GameResult};

pub mod terminal;

pub trait BoardVisualizer {
    fn draw_field(&self, field: &Field);

    fn players_turn(&self, player_turn: bool);

    fn display_result(&self, result: GameResult);
}
