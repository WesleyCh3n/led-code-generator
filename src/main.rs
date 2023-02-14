mod app;
mod mode;
mod strip;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::Vec2 { x: 400., y: 160. }),
        ..Default::default()
    };
    eframe::run_native(
        "Led Code Generator",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
