use eframe::egui::{self, Ui};

use crate::{
    app::App,
    mode::{Mode, ModeConfig},
    strip::Strip,
};

pub fn config_panel(app: &mut App, ui: &mut Ui, index: usize) {
    ui.heading(format!("Config: LED {index}"));
    let Strip {
        mode_cfg, radius, ..
    } = &mut app.strips[index];
    let ModeConfig {
        mode, len, speed, ..
    } = &mut *mode_cfg.lock().unwrap();

    egui::Grid::new("config")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("LED Num");
            ui.add(
                egui::DragValue::new(len)
                    .speed(1.0)
                    .clamp_range(1.0..=100.0),
            );
            ui.end_row();

            ui.label("Raduis");
            ui.add(egui::DragValue::new(radius).speed(1.0));
            ui.end_row();

            ui.label("Speed");
            ui.add(
                egui::DragValue::new(speed)
                    .speed(1.0)
                    .clamp_range(1.0..=1000.0),
            );
            ui.end_row();

            ui.label("Mode");
            egui::ComboBox::new("mode", "")
                .selected_text(format!("{:?}", mode))
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    ui.set_min_width(60.0);
                    ui.selectable_value(mode, Mode::Rainbow, "Rainbow");
                    ui.selectable_value(mode, Mode::Blink, "Blink");
                });
            app.code = mode.get_code();
            ui.end_row();
        });

    ui.separator();
    ui.vertical_centered_justified(|ui| {
        ui.toggle_value(&mut app.show_code, "Show Code");
    });
}
