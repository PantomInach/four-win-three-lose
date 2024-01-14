use crate::field::{Field, GameResult};

mod terminal;

pub(crate) trait BoardVisualizer {
    fn draw_field(field: &Field);

    fn players_turn(player_turn: bool);

    fn display_result(result: GameResult);
}
