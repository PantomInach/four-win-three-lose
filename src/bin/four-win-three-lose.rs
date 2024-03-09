use clap::Parser;
use clap_derive::{Parser, ValueEnum};
use eframe::egui;
use four_win_three_lose::gamelogic::game_handler::GameHandler;
use four_win_three_lose::gui::gui_board::GuiBoard;
use four_win_three_lose::player::{
    computer::ComputerPlayer, human_terminal::HumanTerminal, GamePlayer,
};
use four_win_three_lose::visualizer::terminal::TerminalVisualizer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GameMode {
    /// Run game in terminal. Human VS Human
    TermHVsH,
    /// Run game in terminal. Computer VS Human
    TermCVsH,
    /// Run game in terminal. Human VS Computer
    TermHVsC,
    /// Play inside a graphical user interface.
    GUI,
}

/// Simple Tic-Tac-Toe inspired game on a 4x4 field. If you have four in a row, you win. If you have three in a row, you lose.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    game_mode: Option<GameMode>,
}

fn main() {
    let args = Args::parse();
    match args.game_mode.unwrap_or(GameMode::GUI) {
        GameMode::TermHVsH => {
            start_terminal_game(HumanTerminal::new(false), HumanTerminal::new(true))
        }
        GameMode::TermCVsH => {
            start_terminal_game(ComputerPlayer::new(false), HumanTerminal::new(true))
        }
        GameMode::TermHVsC => {
            start_terminal_game(HumanTerminal::new(false), ComputerPlayer::new(true))
        }
        GameMode::GUI => start_gui(),
    }
}

fn start_terminal_game(player_one: impl GamePlayer, player_two: impl GamePlayer) {
    GameHandler::new(player_one, player_two, TerminalVisualizer {}).play();
}

fn start_gui() {
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
