// ==================== main.rs ====================
use eframe::egui;

mod ui;
mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 480.0]) // A more calculator-like aspect ratio
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(ui::CalculatorApp::default()))),
    )
}
