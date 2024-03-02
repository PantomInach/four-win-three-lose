use eframe::egui::{self, Button};

use crate::{field::Field, Position, FIELD_X, FIELD_Y};

pub struct GuiBoard {
    player_turn: bool,
    field: Field,
    last_button_pressed: Option<Position>,
}
impl eframe::App for GuiBoard {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |central_ui| {
            central_ui.label(players_turn(self.player_turn));
            central_ui.vertical_centered(|ui| {
                egui::Grid::new("id").show(ui, |grid_ui| {
                    for y in 0..FIELD_Y {
                        for x in 0..FIELD_X {
                            if grid_ui
                                .add_sized(
                                    [40.0, 40.0],
                                    Button::new(item_to_string(&self.field.field[y][x])),
                                )
                                .clicked()
                            {
                                self.last_button_pressed = Some(Position { x, y });
                            }
                        }
                        grid_ui.end_row();
                    }
                })
            });
        });
    }
}
impl Default for GuiBoard {
    fn default() -> Self {
        Self {
            player_turn: false,
            field: Field::new(),
            last_button_pressed: None,
        }
    }
}
impl GuiBoard {
    fn reset_last_button_press(&mut self) -> Option<Position> {
        let last_press = self.last_button_pressed;
        self.last_button_pressed = None;
        last_press
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
