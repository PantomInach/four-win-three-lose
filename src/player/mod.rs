use crate::field::Field;
use crate::Position;

pub(crate) mod computer;

pub(crate) trait GamePlayer {
    fn new(player: bool) -> Self;
    fn make_move(&self, field: &Field) -> Position;
    fn invalid_move(&self, field: &Field, pos: Position);
}
