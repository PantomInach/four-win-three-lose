use four_win_three_lose::gamelogic;
pub(crate) use four_win_three_lose::player::computer::ComputerPlayer;
use four_win_three_lose::{player::GamePlayer, visualizer::terminal};

fn main() {
    let player_one = ComputerPlayer::new(false);
    let player_two = ComputerPlayer::new(true);
    let visualizer = terminal::TerminalVisualizer {};

    let mut gamelogic =
        gamelogic::game_handler::GameHandler::from((player_one, player_two, visualizer));
    gamelogic.play();
}
