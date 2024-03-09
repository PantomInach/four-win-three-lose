use log::warn;

use super::GameState;
use crate::{field::Field, player::GamePlayer, visualizer::BoardVisualizer, Position};

pub struct GameHandler<P1, P2, V>
where
    P1: GamePlayer,
    P2: GamePlayer,
    V: BoardVisualizer,
{
    player_one: P1,
    player_two: P2,
    visualizer: V,
    board: Field,
    game_state: GameState,
    player_turn: bool,
}
impl<P1, P2, V> From<(P1, P2, V)> for GameHandler<P1, P2, V>
where
    P1: GamePlayer,
    P2: GamePlayer,
    V: BoardVisualizer,
{
    fn from(value: (P1, P2, V)) -> Self {
        Self {
            player_one: value.0,
            player_two: value.1,
            visualizer: value.2,
            board: Field::new(),
            game_state: GameState::InProgress,
            player_turn: false,
        }
    }
}
impl<P1, P2, V> GameHandler<P1, P2, V>
where
    P1: GamePlayer,
    P2: GamePlayer,
    V: BoardVisualizer,
{
    pub fn new(player_one: P1, player_two: P2, visualizer: V) -> Self {
        (player_one, player_two, visualizer).into()
    }

    fn get_player_move(&mut self) -> Position {
        match self.player_turn {
            false => self.player_one.make_move(&self.board),
            true => self.player_two.make_move(&self.board),
        }
    }

    fn player_move_invalid(&self, pos: Position) {
        match self.player_turn {
            false => self.player_one.invalid_move(&self.board, pos),
            true => self.player_two.invalid_move(&self.board, pos),
        };
    }

    fn update_board(&mut self) {
        self.visualizer.draw_field(&self.board);
        self.visualizer.players_turn(self.player_turn);
    }

    fn display_winner(&mut self) {
        match self.game_state {
            GameState::Finished(result) => self.visualizer.display_result(result),
            GameState::InProgress => panic!("Can't display the game result of a game in progress."),
        }
    }

    fn update_game_state(&mut self) {
        self.update_board();
        self.game_state = self.get_board_state();
        self.player_turn = !self.player_turn;
    }

    fn get_board_state(&self) -> GameState {
        GameState::from(&self.board)
    }

    pub fn play(&mut self) {
        self.update_board();
        while self.game_state == GameState::InProgress {
            let player_move: Position = self.get_player_move();
            if let Err(err) = self
                .board
                .set(player_move.x, player_move.y, self.player_turn)
            {
                warn!(
                    "Player {} tried to make the move {} but got the error: {}",
                    self.player_turn, player_move, err
                );
                self.player_move_invalid(player_move);
                continue;
            }

            // Updates the game state through side effects.
            self.update_game_state();
        }
        self.display_winner();
    }
}
