use std::fmt::Display;

use crate::FIELD_X;
use crate::FIELD_Y;

#[derive(Debug, PartialEq, Eq)]
pub enum FieldErrors {
    PositionNotInField(usize, usize),
    PositionAlreadySet(usize, usize, bool),
    InvalidConstructionSize(usize),
}
impl Display for FieldErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            FieldErrors::PositionNotInField(x, y) => {
                format!("Position ({}, {}) is outside of field.", x, y)
            }
            FieldErrors::PositionAlreadySet(x, y, p) => format!(
                "The place at position ({}, {}) is already occupied from {}.",
                x,
                y,
                if *p { "Player Two" } else { "Player One" }
            ),
            FieldErrors::InvalidConstructionSize(n) => format!(
                "Can't create an {}x{} field from {} elements.",
                FIELD_X, FIELD_Y, n
            ),
        };
        write!(f, "{}", error_message)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum GameResult {
    Draw,
    PlayerOneWins,
    PlayerTwoWins,
}
impl From<bool> for GameResult {
    fn from(value: bool) -> Self {
        match value {
            true => GameResult::PlayerTwoWins,
            false => GameResult::PlayerOneWins,
        }
    }
}
impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            GameResult::Draw => "Draw! Nobody wins.",
            GameResult::PlayerOneWins => "Player One wins!",
            GameResult::PlayerTwoWins => "Player Two wins!",
        };
        write!(f, "{}", message)
    }
}
impl GameResult {
    #[allow(dead_code)]
    fn player_to_result(player: Option<bool>) -> GameResult {
        match player {
            None => GameResult::Draw,
            Some(false) => GameResult::PlayerOneWins,
            Some(true) => GameResult::PlayerTwoWins,
        }
    }

    pub(crate) fn opposite_player(&self) -> GameResult {
        match self {
            GameResult::Draw => GameResult::Draw,
            GameResult::PlayerOneWins => GameResult::PlayerTwoWins,
            GameResult::PlayerTwoWins => GameResult::PlayerOneWins,
        }
    }

    pub(crate) fn better_eq_for_player(&self, other: &GameResult, evaluate_for: bool) -> bool {
        if self == other {
            true
        } else if self == &GameResult::from(evaluate_for) {
            true
        } else {
            other == &GameResult::from(evaluate_for).opposite_player()
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Field {
    pub(super) field: [[Option<bool>; FIELD_X]; FIELD_Y],
}
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_str: String = self
            .field
            .iter()
            .map(|l| {
                l.map(|x| match x {
                    Some(false) => "1",
                    Some(true) => "2",
                    None => "N",
                })
                .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{:?}", field_str)
    }
}
impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
impl From<[Option<bool>; FIELD_X * FIELD_Y]> for Field {
    fn from(value: [Option<bool>; FIELD_X * FIELD_Y]) -> Self {
        let mut field: [[Option<bool>; FIELD_X]; FIELD_Y] = [[None; FIELD_X]; FIELD_Y];
        value.into_iter().enumerate().for_each(|(i, v)| {
            field[i / 4][i % 4] = v;
        });
        Field { field }
    }
}
impl From<[[Option<bool>; FIELD_X]; FIELD_Y]> for Field {
    fn from(value: [[Option<bool>; FIELD_X]; FIELD_Y]) -> Self {
        Field { field: value }
    }
}
impl Field {
    pub fn new() -> Field {
        Field {
            field: [[None; FIELD_X]; FIELD_Y],
        }
    }

    pub(crate) fn mirror_x(&self) -> Field {
        let mut mirrored_field = *self;
        mirrored_field.field.reverse();
        mirrored_field
    }

    pub(crate) fn mirror_y(&self) -> Field {
        let mut mirrored_field = *self;
        mirrored_field.field.iter_mut().for_each(|v| v.reverse());
        mirrored_field
    }

    pub(crate) fn set_pieces(&self) -> usize {
        self.field.iter().map(|v| v.iter().flatten().count()).sum()
    }

    pub(crate) fn set(&mut self, x: usize, y: usize, player: bool) -> Result<(), FieldErrors> {
        if let Some(state) = self.field.get_mut(y).and_then(|v| v.get_mut(x)) {
            match state {
                Some(p) => Err(FieldErrors::PositionAlreadySet(x, y, *p)),
                None => {
                    *state = Some(player);
                    Ok(())
                }
            }
        } else {
            Err(FieldErrors::PositionNotInField(x, y))
        }
    }

    pub(crate) fn force_set(
        &mut self,
        x: usize,
        y: usize,
        item: Option<bool>,
    ) -> Result<(), FieldErrors> {
        if let Some(state) = self.field.get_mut(y).and_then(|v| v.get_mut(x)) {
            *state = item;
            Ok(())
        } else {
            Err(FieldErrors::PositionNotInField(x, y))
        }
    }

    pub(crate) fn winner(&self) -> Option<bool> {
        if let Some(winner) = self.field.iter().find_map(|v| {
            (v[0].is_some() && v[0] == v[1] && v[0] == v[2] && v[0] == v[3]).then_some(v[0])
        }) {
            return winner;
        }

        if let Some(winner) = (0..FIELD_X).find_map(|x| {
            (self.field[0][x].is_some()
                && self.field[0][x] == self.field[1][x]
                && self.field[0][x] == self.field[2][x]
                && self.field[0][x] == self.field[3][x])
                .then_some(self.field[0][x])
        }) {
            return winner;
        }

        if self.field[0][0].is_some()
            && self.field[0][0] == self.field[1][1]
            && self.field[0][0] == self.field[2][2]
            && self.field[0][0] == self.field[3][3]
        {
            return self.field[0][0];
        }
        if self.field[0][3].is_some()
            && self.field[0][3] == self.field[1][2]
            && self.field[0][3] == self.field[2][1]
            && self.field[0][3] == self.field[3][0]
        {
            return self.field[0][3];
        }

        None
    }

    pub(crate) fn loser(&self) -> Option<bool> {
        if let Some(p) = [(0, 0), (0, 1), (1, 0), (1, 1)]
            .into_iter()
            .find_map(|(dx, dy)| {
                if self.field[1 + dy][1 + dx].is_some()
                    && self.field[1 + dy][1 + dx] == self.field[dy][dx]
                    && self.field[1 + dy][1 + dx] == self.field[2 + dy][2 + dx]
                {
                    self.field[1 + dy][1 + dx]
                } else if self.field[1 + dy][1 + dx].is_some()
                    && self.field[1 + dy][1 + dx] == self.field[2 + dy][dx]
                    && self.field[1 + dy][1 + dx] == self.field[dy][2 + dx]
                {
                    self.field[1 + dy][1 + dx]
                } else {
                    None
                }
            })
        {
            return Some(p);
        }

        if let Some(loser) = (0..FIELD_X).find_map(|x| {
            (self.field[1][x].is_some()
                && self.field[1][x] == self.field[2][x]
                && ((self.field[1][x] == self.field[0][x])
                    ^ (self.field[1][x] == self.field[3][x])))
                .then_some(self.field[1][x])
        }) {
            return loser;
        }
        if let Some(loser) = (0..FIELD_X).find_map(|x| {
            (self.field[x][1].is_some()
                && self.field[x][1] == self.field[x][2]
                && ((self.field[x][1] == self.field[x][0])
                    ^ (self.field[x][1] == self.field[x][3])))
                .then_some(self.field[x][1])
        }) {
            return loser;
        }

        None
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;

    #[test]
    fn better_eq_for_player() {
        let w1 = GameResult::PlayerOneWins;
        let w2 = GameResult::PlayerTwoWins;
        let d = GameResult::Draw;

        assert_eq!(true, w1.better_eq_for_player(&w1, false));
        assert_eq!(true, w1.better_eq_for_player(&w2, false));
        assert_eq!(true, w1.better_eq_for_player(&d, false));
        assert_eq!(true, w1.better_eq_for_player(&w1, true));
        assert_eq!(false, w1.better_eq_for_player(&w2, true));
        assert_eq!(false, w1.better_eq_for_player(&d, true));

        assert_eq!(false, d.better_eq_for_player(&w1, false));
        assert_eq!(true, d.better_eq_for_player(&w2, false));
        assert_eq!(true, d.better_eq_for_player(&d, false));
        assert_eq!(true, d.better_eq_for_player(&w1, true));
        assert_eq!(false, d.better_eq_for_player(&w2, true));
        assert_eq!(true, d.better_eq_for_player(&d, true));

        assert_eq!(false, w2.better_eq_for_player(&w1, false));
        assert_eq!(true, w2.better_eq_for_player(&w2, false));
        assert_eq!(false, w2.better_eq_for_player(&d, false));
        assert_eq!(true, w2.better_eq_for_player(&w1, true));
        assert_eq!(true, w2.better_eq_for_player(&w2, true));
        assert_eq!(true, w2.better_eq_for_player(&d, true));
    }

    fn create_sample_field_empty() -> Field {
        Field::from([
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
        ])
    }

    #[test]
    fn field_into() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;

        let mut arr = [None; 16];
        let field: Field = arr.into();
        let empty_f = Field::from([[n, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(empty_f.field, field.field);

        arr[0] = Some(false);
        arr[4] = Some(true);
        arr[8] = Some(false);
        arr[12] = Some(true);
        let field: Field = arr.into();
        let should_be = Field::from([[f, n, n, n], [t, n, n, n], [f, n, n, n], [t, n, n, n]]);

        assert_eq!(should_be.field, field.field);

        arr[3] = Some(true);
        arr[7] = Some(true);
        arr[11] = Some(true);
        arr[15] = Some(true);
        let field: Field = arr.into();
        let should_be = Field::from([[f, n, n, t], [t, n, n, t], [f, n, n, t], [t, n, n, t]]);
        assert_eq!(should_be.field, field.field);
    }

    #[test]
    fn set() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let sample_1 = Field::from([[f, n, n, n], [t, n, n, n], [f, n, n, n], [t, n, n, n]]);

        let mut field = create_sample_field_empty();
        assert_eq!(Ok(()), field.set(0, 0, false));
        assert_eq!(Ok(()), field.set(0, 1, true));
        assert_eq!(Ok(()), field.set(0, 2, false));
        assert_eq!(Ok(()), field.set(0, 3, true));
        assert_eq!(sample_1.field, field.field);

        assert_eq!(
            Err(FieldErrors::PositionNotInField(1, FIELD_Y)),
            field.set(1, FIELD_Y, true)
        );
        assert_eq!(
            Err(FieldErrors::PositionNotInField(FIELD_X, 1)),
            field.set(FIELD_X, 1, false)
        );
        assert_eq!(sample_1.field, field.field);

        assert_eq!(
            Err(FieldErrors::PositionAlreadySet(0, 3, true)),
            field.set(0, 3, true)
        );
        assert_eq!(
            Err(FieldErrors::PositionAlreadySet(0, 3, true)),
            field.set(0, 3, false)
        );
        assert_eq!(
            Err(FieldErrors::PositionAlreadySet(0, 0, false)),
            field.set(0, 0, false)
        );
        assert_eq!(
            Err(FieldErrors::PositionAlreadySet(0, 0, false)),
            field.set(0, 0, true)
        );
        assert_eq!(sample_1.field, field.field);
    }

    #[test]
    fn test_set_pieces() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let sample_1 = Field::from([[f, n, n, t], [t, n, n, t], [f, n, n, t], [t, n, n, t]]);
        assert_eq!(sample_1.set_pieces(), 8_usize);
    }

    #[test]
    fn force_set() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let sample_1 = Field::from([[f, n, n, n], [t, n, n, n], [f, n, n, n], [t, n, n, n]]);
        let mut field = sample_1.clone();

        assert_eq!(
            Err(FieldErrors::PositionNotInField(1, FIELD_Y)),
            field.set(1, FIELD_Y, true)
        );
        assert_eq!(
            Err(FieldErrors::PositionNotInField(FIELD_X, 1)),
            field.set(FIELD_X, 1, false)
        );
        assert_eq!(sample_1.field, field.field);

        assert_eq!(Ok(()), field.force_set(0, 0, None));
        assert_eq!(Ok(()), field.force_set(0, 1, None));
        assert_eq!(Ok(()), field.force_set(0, 2, None));
        assert_eq!(Ok(()), field.force_set(0, 3, None));
        assert_eq!(create_sample_field_empty().field, field.field);

        let x = [
            [Some(true), None, None, None],
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
        ];
        assert_eq!(Ok(()), field.force_set(0, 0, Some(true)));
        assert_eq!(x, field.field);
        assert_eq!(Ok(()), field.force_set(0, 0, Some(true)));
        assert_eq!(x, field.field);
        let x = [
            [Some(false), None, None, None],
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
        ];
        assert_eq!(Ok(()), field.force_set(0, 0, Some(false)));
        assert_eq!(x, field.field);
    }

    #[test]
    fn winner() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let field = Field::from([[t, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(field.winner(), n);
        let field = Field::from([[t, n, t, t], [t, n, n, n], [t, n, t, n], [n, t, t, t]]);
        assert_eq!(field.winner(), n);
        let field = Field::from([[t, f, t, t], [t, t, t, n], [t, t, f, n], [f, n, n, t]]);
        assert_eq!(field.winner(), n);
        let field = Field::from([[t, n, n, n], [t, n, n, n], [t, n, n, n], [t, n, n, n]]);
        assert_eq!(field.winner(), t);
        let field = Field::from([[f, n, n, n], [f, n, n, n], [f, n, n, n], [f, n, n, n]]);
        assert_eq!(field.winner(), f);
        let field = Field::from([[t, t, t, t], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(field.winner(), t);
        let field = Field::from([[n, n, n, n], [n, n, n, n], [n, n, n, n], [t, t, t, t]]);
        assert_eq!(field.winner(), t);
        let field = Field::from([[t, n, n, n], [n, t, n, n], [n, n, t, n], [n, n, n, t]]);
        assert_eq!(field.winner(), t);
        let field = Field::from([[n, n, n, t], [n, n, t, n], [n, t, n, n], [t, n, n, n]]);
        assert_eq!(field.winner(), t);
    }

    #[test]
    fn loser() {
        let t = Some(true);
        let f = Some(false);
        let n: Option<bool> = None;
        let field = Field::from([[n, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(field.loser(), None);
        let field = Field::from([[t, t, n, t], [t, n, t, t], [f, n, f, f], [f, f, t, f]]);
        assert_eq!(field.loser(), None);
        let field = Field::from([[t, n, n, t], [n, t, t, n], [n, n, n, n], [t, n, n, t]]);
        assert_eq!(field.loser(), None);

        let field = Field::from([[t, t, t, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, t, t, t], [n, n, n, n], [n, n, n, n], [n, n, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, t, n, n], [n, t, n, n], [n, t, n, n], [n, n, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, n, n, n], [n, t, n, n], [n, t, n, n], [n, t, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[t, n, n, n], [n, t, n, n], [n, n, t, n], [n, n, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, n, n, n], [n, t, n, n], [n, n, t, n], [n, n, n, t]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, t, n, n], [n, n, t, n], [n, n, n, t], [n, n, n, n]]);
        assert_eq!(field.loser(), t);
        let field = Field::from([[n, n, n, n], [n, n, n, t], [n, n, t, n], [n, t, n, n]]);
        assert_eq!(field.loser(), t);
    }

    #[test]
    fn test_manipulations() {
        let f = Some(false);
        let t = Some(true);
        let n: Option<bool> = None;

        let field = Field::from([[t, f, n, n], [n, t, f, n], [n, n, t, f], [f, n, n, t]]);
        let mirror_x = Field::from([[f, n, n, t], [n, n, t, f], [n, t, f, n], [t, f, n, n]]);
        assert_eq!(field.mirror_x(), mirror_x);

        let mirror_y = Field::from([[n, n, f, t], [n, f, t, n], [f, t, n, n], [t, n, n, f]]);
        assert_eq!(field.mirror_y(), mirror_y);
    }
}
