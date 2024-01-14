use crate::field::Field;
use crate::Position;

pub(crate) mod computer;

trait GamePlayer {
    fn new(player: bool) -> Self;
    fn make_move(&self, field: &Field) -> Position;
}
