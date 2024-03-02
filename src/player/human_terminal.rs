use super::GamePlayer;
use super::Position;
use crate::field::Field;
use itertools::Itertools;
use std::io;

pub struct HumanTerminal {}
impl GamePlayer for HumanTerminal {
    fn new(_player: bool) -> Self {
        HumanTerminal {}
    }

    fn make_move(&self, _: &Field) -> Position {
        println!("Please give a valid move of pattern 'x y':");
        let mut pos: Result<Position, String>;
        loop {
            pos = Position::try_from(get_user_input());
            if pos.is_err() {
                println!("Your inputs needs to have the form 'x y'. Try again:");
            } else {
                return pos.unwrap();
            }
        }
    }

    fn invalid_move(&self, field: &Field, pos: Position) {
        if let Some(possible_moves) = field.possible_moves() {
            let moves = possible_moves
                .iter()
                .map(|p| p.to_string())
                .join(", ")
                .replace("(", "'")
                .replace(")", "'");
            println!(
                "Can't make the move {}. Try again. Possible moves: {}",
                pos, moves
            );
        } else {
            unreachable!(
                "Can't make a move on a new field. Game should have been already terminated."
            )
        }
    }
}

fn get_user_input() -> String {
    let input = &mut String::new();
    let _ = io::stdin().read_line(input);
    input.to_owned()
}
