use crate::field::Field;
use crate::Position;

pub mod computer;
pub mod human_terminal;

pub trait GamePlayer {
    fn new(player: bool) -> Self;
    fn make_move(&self, field: &Field) -> Position;
    fn invalid_move(&self, field: &Field, pos: Position);
}
