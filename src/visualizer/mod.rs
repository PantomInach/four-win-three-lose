use crate::field::{Field, GameResult};

mod terminal;

pub(crate) trait BoardVisualizer {
    fn draw_field(&self, field: &Field);

    fn players_turn(&self, player_turn: bool);

    fn display_result(&self, result: GameResult);
}
