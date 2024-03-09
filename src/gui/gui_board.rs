use eframe::egui::{self, Button};

use crate::{
    field::Field,
    gamelogic::GameState,
    player::{computer::ComputerPlayer, GamePlayer},
    FIELD_X, FIELD_Y,
};

pub enum Player {
    Human,
    Computer(ComputerPlayer),
}

pub struct GuiBoard {
    player_turn: bool,
    field: Field,
    game_state: GameState,
    player_one: Player,
    player_two: Player,
}
impl eframe::App for GuiBoard {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.game_state == GameState::InProgress {
            self.handle_computer_player();
        }

        egui::CentralPanel::default().show(ctx, |central_ui| {
            central_ui.label(players_turn(self.player_turn));
            central_ui.vertical_centered(|ui| {
                egui::Grid::new("id").show(ui, |grid_ui| {
                    for y in 0..FIELD_Y {
                        for x in 0..FIELD_X {
                            let button = grid_ui.add_sized(
                                [40.0, 40.0],
                                Button::new(item_to_string(&self.field.field[y][x])),
                            );
                            if self.game_state == GameState::InProgress
                                && button.clicked()
                                && self.field.field[y][x].is_none()
                            {
                                self.handle_button_press(x, y);
                            }
                        }
                        grid_ui.end_row();
                    }
                })
            });
        });

        if let GameState::Finished(result) = self.game_state {
            egui::Window::new(result.to_string())
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if ui.button("Ok").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
        }
        if ctx.input(|i| i.viewport().close_requested()) {
            // The application will be closed.
        }
    }
}
impl Default for GuiBoard {
    fn default() -> Self {
        Self {
            player_turn: false,
            field: Field::new(),
            game_state: GameState::InProgress,
            player_one: Player::Human,
            player_two: Player::Human,
        }
    }
}
impl GuiBoard {
    pub fn new(player_one: Player, player_two: Player) -> Self {
        GuiBoard {
            player_turn: false,
            field: Field::new(),
            game_state: GameState::InProgress,
            player_one,
            player_two,
        }
    }

    pub(crate) fn get_player(&self, player: bool) -> &Player {
        match player {
            true => &self.player_two,
            false => &self.player_one,
        }
    }

    fn handle_computer_player(&mut self) {
        if let Player::Computer(com) = self.get_player(self.player_turn) {
            let com_move = com.make_move(&self.field);
            if let Err(error) = self.field.set(com_move.x, com_move.y, self.player_turn) {
                unreachable!("Computer made an invalid move: {}", error);
            }
            self.player_turn = !self.player_turn;
        }
    }

    fn handle_button_press(&mut self, x: usize, y: usize) {
        if let Err(error) = self.field.set(x, y, self.player_turn) {
            panic!("Error occured while pressing a button: {}", error);
        }
        self.game_state = GameState::from(&self.field).into();

        self.player_turn = !self.player_turn;
    }
}

fn players_turn(player: bool) -> String {
    match player {
        true => "It's player two's turn.",
        false => "It's player one's turn.",
    }
    .to_string()
}

fn item_to_string(field_item: &Option<bool>) -> String {
    match field_item {
        Some(false) => "X",
        Some(true) => "O",
        None => " ",
    }
    .to_string()
}
