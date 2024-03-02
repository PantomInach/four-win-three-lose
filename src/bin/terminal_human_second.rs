pub(crate) use four_win_three_lose::player::human_terminal::HumanTerminal;
use four_win_three_lose::{gamelogic, player::computer::ComputerPlayer};
use four_win_three_lose::{player::GamePlayer, visualizer::terminal};

fn main() {
    let player_one = ComputerPlayer::new(true);
    let player_two = HumanTerminal::new(false);
    let visualizer = terminal::TerminalVisualizer {};

    let mut gamelogic =
        gamelogic::game_handler::GameHandler::from((player_one, player_two, visualizer));
    gamelogic.play();
}
