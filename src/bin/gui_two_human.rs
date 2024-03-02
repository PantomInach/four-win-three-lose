// use four_win_three_lose::gamelogic;
// use four_win_three_lose::player::GamePlayer;

use eframe::egui;
use four_win_three_lose::gui::gui_board::GuiBoard;

fn main() {
    // let player_one = HumanTerminal::new(false);
    // let player_two = HumanTerminal::new(true);
    // let visualizer = terminal::TerminalVisualizer {};

    // let mut gamelogic =
    //     gamelogic::game_handler::GameHandler::from((player_one, player_two, visualizer));
    // gamelogic.play();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 480.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "4Win3Loss",
        options,
        Box::new(|_cc| Box::<GuiBoard>::default()),
    );
}
