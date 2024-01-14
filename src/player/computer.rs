use super::GamePlayer;
use crate::field::GameResult;
use crate::player::Field;
use crate::NextBestMove;
use crate::Position;
use crate::FIELD_X;
use crate::FIELD_Y;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

struct ComputerPlayer {
    player: bool,
}
impl GamePlayer for ComputerPlayer {
    fn new(player: bool) -> Self {
        ComputerPlayer { player }
    }

    fn make_move(&self, field: &crate::field::Field) -> Position {
        let mut f = field.clone();
        match f.possible_non_symmetrical_moves() {
            Some(moves) => {
                self.get_best_move(&mut f, &moves)
            },
            None => panic!("The computer player is expected to do a move, but no moves available. Current Field:\n{}", field),
        }
    }
}
impl ComputerPlayer {
    /// This is just a reimplementation of [Field::brute_force_game_state_recursivly].
    fn get_best_move(&self, field: &mut Field, moves: &Vec<Position>) -> Position {
        let move_selector = Field::possible_moves;
        let mut game_cash = HashMap::new();
        let best_move: NextBestMove = ((0, 0), GameResult::from(self.player).opposite_player());

        moves
            .iter()
            .fold_while(best_move, |(best_pos, game_res), pos| {
                let _ = field.set(pos.0, pos.1, self.player);
                let rec_res = field.brute_force_game_state_recursivly(
                    !self.player,
                    !self.player,
                    &move_selector,
                    &mut game_cash,
                );
                let _ = field.force_set(pos.0, pos.1, None);
                if rec_res == GameResult::from(self.player) {
                    Done((*pos, rec_res))
                } else if rec_res.better_eq_for_player(&game_res, self.player) {
                    Continue((*pos, rec_res))
                } else {
                    Continue((best_pos, game_res))
                }
            })
            .into_inner()
            .0
    }
}

/// Implement brute force utils for solving a field state.
impl Field {
    pub fn possible_moves(&self) -> Option<Vec<Position>> {
        let possible_moves: Vec<Position> = (0..FIELD_X)
            .flat_map(|x| {
                (0..FIELD_Y)
                    .map(move |y| (x, y))
                    .filter(|(x, y)| self.field[*y][*x].is_none())
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
                    let _ = field.set(pos.0, pos.1, false);
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
        )
    }

    fn brute_force_game_state_recursivly<F>(
        &mut self,
        evaluate_for: bool,
        player_turn: bool,
        move_selector: &F,
        game_cash: &mut HashMap<[[Option<bool>; FIELD_X]; FIELD_Y], GameResult>,
    ) -> GameResult
    where
        F: Fn(&Field) -> Option<Vec<Position>>,
    {
        if let Some(winner) = self.winner() {
            return GameResult::from(winner);
        }
        if let Some(loser) = self.loser() {
            return GameResult::from(loser).opposite_player();
        }

        if let Some(res) = game_cash.get(&self.field) {
            return *res;
        }

        let best_move: NextBestMove = ((0, 0), GameResult::from(evaluate_for).opposite_player());

        let res = match move_selector(self) {
            None => GameResult::Draw,
            Some(possible_moves) => {
                possible_moves
                    .iter()
                    .fold_while(best_move, |(best_pos, game_res), pos| {
                        let _ = self.set(pos.0, pos.1, player_turn);
                        let rec_res = self.brute_force_game_state_recursivly(
                            !evaluate_for,
                            !player_turn,
                            move_selector,
                            game_cash,
                        );
                        let _ = self.force_set(pos.0, pos.1, None);

                        if rec_res == GameResult::from(evaluate_for) {
                            Done((*pos, rec_res))
                        } else if rec_res.better_eq_for_player(&game_res, evaluate_for) {
                            Continue((*pos, rec_res))
                        } else {
                            Continue((best_pos, game_res))
                        }
                    })
                    .into_inner()
                    .1
            }
        };
        game_cash.insert(self.field, res);
        res
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
        assert_eq!(player.make_move(&field), (2, 3));
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
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3)
            ])
        );
        let field = Field::from([[f, f, f, f], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(
            field.possible_moves(),
            Some(vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3)
            ])
        );
        let field = Field::from([[f, n, n, n], [n, f, n, n], [n, n, f, n], [n, n, n, f]]);
        assert_eq!(
            field.possible_moves(),
            Some(vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (2, 3),
                (3, 0),
                (3, 1),
                (3, 2),
            ])
        );
        let field = Field::from([[f, f, f, f], [f, f, f, f], [f, f, f, f], [f, f, f, n]]);
        assert_eq!(field.possible_moves(), Some(vec![(3, 3),]));
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
                (0, 0),
                (0, 1),
                // (0, 2),
                // (0, 3),
                (1, 0),
                (1, 1),
                // (1, 2),
                // (1, 3),
                // (2, 0),
                // (2, 1),
                // (2, 2),
                // (2, 3),
                // (3, 0),
                // (3, 1),
                // (3, 2),
                // (3, 3)
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
