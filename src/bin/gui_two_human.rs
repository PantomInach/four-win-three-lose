use eframe::egui;
use four_win_three_lose::gui::gui_board::GuiBoard;

fn main() {
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
