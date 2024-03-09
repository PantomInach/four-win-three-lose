use crate::field::Field;
use crate::GameResult;

pub mod terminal;

pub trait BoardVisualizer {
    fn draw_field(&mut self, field: &Field);

    fn players_turn(&mut self, player_turn: bool);

    fn display_result(&mut self, result: GameResult);
}
