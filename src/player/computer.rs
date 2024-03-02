use super::GamePlayer;
use crate::player::Field;
use crate::GameResult;
use crate::NextBestMove;
use crate::Position;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

pub struct ComputerPlayer {
    player: bool,
}
impl GamePlayer for ComputerPlayer {
    fn new(player: bool) -> Self {
        ComputerPlayer { player }
    }

    fn make_move(&self, field: &Field) -> Position {
        let mut f = *field;
        match f.possible_non_symmetrical_moves() {
            Some(moves) => {
                self.get_best_move(&mut f, &moves)
            },
            None => panic!("The computer player is expected to do a move, but no moves available. Current Field:\n{}", field),
        }
    }

    fn invalid_move(&self, _: &Field, _: Position) {
        panic!("The computer can't make an invalid move.")
    }
}
impl ComputerPlayer {
    /// This is just a reimplementation of [Field::brute_force_game_state_recursivly].
    fn get_best_move(&self, field: &mut Field, moves: &[Position]) -> Position {
        let move_selector = Field::possible_moves;
        let mut game_cash = HashMap::new();
        let best_move: NextBestMove = (
            (0, 0).into(),
            GameResult::from(self.player).opposite_player(),
            0,
        );

        moves
            .iter()
            .fold_while(best_move, |(best_pos, game_res, depth), pos| {
                let _ = field.set(pos.x, pos.y, self.player);
                let rec_res = field.brute_force_game_state_recursivly(
                    !self.player,
                    !self.player,
                    &move_selector,
                    &mut game_cash,
                    field.set_pieces(),
                );
                let _ = field.force_set(pos.x, pos.y, None);
                if rec_res.0 == GameResult::from(self.player) {
                    Done((*pos, rec_res.0, rec_res.1))
                } else if rec_res.0 == GameResult::Draw {
                    Continue((*pos, rec_res.0, rec_res.1))
                } else if rec_res.1 > depth {
                    Continue((*pos, rec_res.0, rec_res.1))
                } else {
                    Continue((best_pos, game_res, depth))
                }
            })
            .into_inner()
            .0
    }
}

#[cfg(test)]
mod test_computer_player {
    use super::*;

    #[test]
    fn test_make_move() {
        let player: ComputerPlayer = ComputerPlayer::new(false);
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;

        let field = Field::from([[f, f, t, t], [t, t, f, f], [f, t, n, t], [f, f, n, f]]);
        assert_eq!(player.make_move(&field), (2, 3).into());
    }
}
