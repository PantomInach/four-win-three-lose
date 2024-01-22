use crate::field::Field;
use crate::GameResult;
use crate::NextBestMove;
use crate::Position;
use crate::FIELD_X;
use crate::FIELD_Y;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

/// Implement brute force utils for solving a field state.
impl Field {
    pub fn possible_moves(&self) -> Option<Vec<Position>> {
        let possible_moves: Vec<Position> = (0..FIELD_X)
            .flat_map(|x| {
                (0..FIELD_Y)
                    .map(move |y| Position::from((x, y)))
                    .filter(|pos| self.field[pos.y][pos.x].is_none())
            })
            .collect();
        (!possible_moves.is_empty()).then_some(possible_moves)
    }

    pub fn possible_non_symmetrical_moves(&self) -> Option<Vec<Position>> {
        let mut res: Vec<(Position, Field)> = Vec::new();
        if let Some(positions) = self.possible_moves() {
            positions
                .into_iter()
                .map(|pos| {
                    let mut field = *self;
                    let _ = field.set(pos.x, pos.y, false);
                    (pos, field)
                })
                .for_each(|(pos, field)| {
                    if !res.iter().any(|(_, f)| {
                        f == &field.mirror_x()
                            || f == &field.mirror_y()
                            || f == &field.mirror_x().mirror_y()
                    }) {
                        res.push((pos, field));
                    }
                });
        };
        if res.is_empty() {
            None
        } else {
            Some(res.into_iter().map(|(p, _)| p).collect())
        }
    }

    /// The additional level of redirection makes the program drastically slower. Probably some
    /// optimization is not done by the compiler.
    pub fn possible_moves_symmetrical_if_sparse(&self) -> Option<Vec<Position>> {
        if self.set_pieces() > 5 {
            self.possible_moves()
        } else {
            self.possible_moves_symmetrical_if_sparse()
        }
    }

    /// Setting [evaluate_for] and [player_turn] to opposites will make the player try to lose the
    /// game.
    pub fn brute_force_game_state<F>(
        &mut self,
        evaluate_for: bool,
        player_turn: bool,
        move_selector: &F,
    ) -> GameResult
    where
        F: Fn(&Field) -> Option<Vec<Position>>,
    {
        self.brute_force_game_state_recursivly(
            evaluate_for,
            player_turn,
            move_selector,
            &mut HashMap::new(),
            0,
        )
        .0
    }

    pub(crate) fn brute_force_game_state_recursivly<F>(
        &mut self,
        evaluate_for: bool,
        player_turn: bool,
        move_selector: &F,
        game_cash: &mut HashMap<[[Option<bool>; FIELD_X]; FIELD_Y], GameResult>,
        nth_move: usize,
    ) -> (GameResult, usize)
    where
        F: Fn(&Field) -> Option<Vec<Position>>,
    {
        if let Some(winner) = self.winner() {
            return (GameResult::from(winner), nth_move);
        }
        if let Some(loser) = self.loser() {
            return (GameResult::from(loser).opposite_player(), nth_move);
        }

        if let Some(res) = game_cash.get(&self.field) {
            return (*res, nth_move);
        }

        let best_move: NextBestMove = (
            (0, 0).into(),
            GameResult::from(evaluate_for).opposite_player(),
            0,
        );

        let res: (Position, GameResult, usize) = match move_selector(self) {
            None => ((0, 0).into(), GameResult::Draw, nth_move + 1),
            Some(possible_moves) => possible_moves
                .iter()
                .fold_while(best_move, |(best_pos, game_res, depth), pos| {
                    let _ = self.set(pos.x, pos.y, player_turn);
                    let rec_res = self.brute_force_game_state_recursivly(
                        !evaluate_for,
                        !player_turn,
                        move_selector,
                        game_cash,
                        nth_move + 1,
                    );
                    let _ = self.force_set(pos.x, pos.y, None);

                    if rec_res.0 == GameResult::from(evaluate_for) {
                        Done((*pos, rec_res.0, rec_res.1))
                    } else if rec_res.0.better_eq_for_player(&game_res, evaluate_for) {
                        Continue((*pos, rec_res.0, rec_res.1))
                    } else {
                        Continue((best_pos, game_res, depth))
                    }
                })
                .into_inner(),
        };
        game_cash.insert(self.field, res.1);
        (res.1, res.2)
    }
}

#[cfg(test)]
mod test_brute_force {
    use super::*;

    #[test]
    fn possible_moves() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let field = Field::from([[n, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(
            field.possible_moves(),
            Some(vec![
                (0, 0).into(),
                (0, 1).into(),
                (0, 2).into(),
                (0, 3).into(),
                (1, 0).into(),
                (1, 1).into(),
                (1, 2).into(),
                (1, 3).into(),
                (2, 0).into(),
                (2, 1).into(),
                (2, 2).into(),
                (2, 3).into(),
                (3, 0).into(),
                (3, 1).into(),
                (3, 2).into(),
                (3, 3).into()
            ])
        );
        let field = Field::from([[f, f, f, f], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(
            field.possible_moves(),
            Some(vec![
                (0, 1).into(),
                (0, 2).into(),
                (0, 3).into(),
                (1, 1).into(),
                (1, 2).into(),
                (1, 3).into(),
                (2, 1).into(),
                (2, 2).into(),
                (2, 3).into(),
                (3, 1).into(),
                (3, 2).into(),
                (3, 3).into()
            ])
        );
        let field = Field::from([[f, n, n, n], [n, f, n, n], [n, n, f, n], [n, n, n, f]]);
        assert_eq!(
            field.possible_moves(),
            Some(vec![
                (0, 1).into(),
                (0, 2).into(),
                (0, 3).into(),
                (1, 0).into(),
                (1, 2).into(),
                (1, 3).into(),
                (2, 0).into(),
                (2, 1).into(),
                (2, 3).into(),
                (3, 0).into(),
                (3, 1).into(),
                (3, 2).into(),
            ])
        );
        let field = Field::from([[f, f, f, f], [f, f, f, f], [f, f, f, f], [f, f, f, n]]);
        assert_eq!(field.possible_moves(), Some(vec![(3, 3).into(),]));
        let field = Field::from([[f, f, f, f], [f, f, f, f], [f, f, f, f], [f, f, f, t]]);
        assert_eq!(field.possible_moves(), None);
    }

    #[test]
    fn test_possible_moves_symetric() {
        let _t = Some(true);
        let _f = Some(false);
        let n: Option<bool> = None;

        let field = Field::from([[n, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(
            field.possible_non_symmetrical_moves(),
            Some(vec![
                (0, 0).into(),
                (0, 1).into(),
                (1, 0).into(),
                (1, 1).into(),
            ])
        );
    }

    #[test]
    fn test_brute_force_game_state() {
        let f = Some(false);
        let t = Some(true);
        let n: Option<bool> = None;

        let move_selector = Field::possible_moves;

        let mut field = Field::from([[t, t, f, f], [f, f, t, t], [t, t, f, f], [f, f, t, t]]);
        assert_eq!(
            field.brute_force_game_state(false, false, &move_selector),
            GameResult::Draw
        );
        let mut field = Field::from([[t, t, f, f], [f, f, t, t], [t, t, f, f], [f, f, t, n]]);
        assert_eq!(
            field.brute_force_game_state(false, true, &move_selector),
            GameResult::Draw
        );

        let mut field = Field::from([[t, t, f, f], [f, f, t, t], [t, t, f, f], [f, n, t, n]]);
        assert_eq!(
            field.brute_force_game_state(false, false, &move_selector),
            GameResult::Draw
        );
        let mut field = Field::from([[t, t, f, f], [f, f, t, t], [t, t, f, f], [f, n, t, n]]);
        assert_eq!(
            field.brute_force_game_state(true, true, &move_selector),
            GameResult::PlayerTwoWins
        );

        let mut field = Field::from([[t, f, t, f], [t, t, f, t], [f, t, f, n], [t, f, n, n]]);
        assert_eq!(
            field.brute_force_game_state(true, true, &move_selector),
            GameResult::PlayerTwoWins
        );
        let mut field = Field::from([[t, f, t, f], [t, t, f, t], [f, t, f, n], [t, f, n, n]]);
        assert_eq!(
            field.brute_force_game_state(false, false, &move_selector),
            GameResult::PlayerTwoWins
        );

        let mut field = Field::from([[t, f, t, f], [t, t, f, t], [f, t, n, n], [t, f, n, n]]);
        assert_eq!(
            field.brute_force_game_state(false, false, &move_selector),
            GameResult::PlayerOneWins
        );
        let mut field = Field::from([[t, f, t, f], [t, t, f, t], [f, t, n, n], [t, f, n, n]]);
        assert_eq!(
            field.brute_force_game_state(true, true, &move_selector),
            GameResult::PlayerOneWins
        );
    }
}
