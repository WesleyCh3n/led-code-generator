use eframe::egui::plot::{MarkerShape, Plot, Points};
use eframe::egui::ScrollArea;
use eframe::{egui, epaint::Color32};

use crate::mode::Mode;
use crate::strip::Strip;

pub struct App {
    strips: Vec<Strip>,
    curr: Option<usize>,
    show_code: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            strips: Vec::new(),
            curr: None,
            show_code: false,
        }
    }
}

impl eframe::App for App {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Close the menu").clicked() {
                        ui.close_menu();
                    }
                });
            })
        });
        egui::SidePanel::left("left panel")
            .default_width(230.0)
            .show(ctx, |ui| {
                ui.heading("Strips");
                ScrollArea::horizontal()
                    .vscroll(true)
                    .auto_shrink([false; 2])
                    .max_height(200.0)
                    .show(ui, |ui| {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::Center)
                                .with_cross_justify(true),
                            |ui| {
                                for i in 0..self.strips.len() {
                                    ui.selectable_value(
                                        &mut self.curr,
                                        Some(i),
                                        format!("LED {i}"),
                                    );
                                }
                            },
                        );
                    });

                ui.columns(2, |cols| {
                    if cols[0].button("+").clicked() {
                        self.strips.push(Strip::default())
                    }
                    cols[1].add_enabled_ui(self.curr.is_some(), |ui| {
                        if ui.button("-").clicked() {
                            self.strips.remove(self.curr.unwrap());
                            self.curr = None;
                        }
                    });
                });
                ui.separator();

                // read curr and show
                if let Some(i) = self.curr {
                    ui.heading(format!("Config: LED {i}"));
                    egui::Grid::new("config")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("LED Num");
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.strips[i]
                                        .mode_cfg
                                        .lock()
                                        .unwrap()
                                        .len,
                                )
                                .speed(1.0)
                                .clamp_range(1.0..=100.0),
                            );
                            ui.end_row();
                            ui.label("Gap");
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.strips[i].radius,
                                )
                                .speed(1.0),
                            );
                            ui.end_row();
                            ui.label("Speed");
                            ui.add(
                                egui::DragValue::new(
                                    &mut self.strips[i]
                                        .mode_cfg
                                        .lock()
                                        .unwrap()
                                        .speed,
                                )
                                .speed(1.0),
                            );
                            ui.end_row();
                            ui.label("Mode");
                            egui::ComboBox::new("mode", "")
                                .selected_text(format!(
                                    "{:?}",
                                    self.strips[i]
                                        .mode_cfg
                                        .lock()
                                        .unwrap()
                                        .mode
                                ))
                                .show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.set_min_width(60.0);
                                    ui.selectable_value(
                                        &mut self.strips[i]
                                            .mode_cfg
                                            .lock()
                                            .unwrap()
                                            .mode,
                                        Mode::Rainbow,
                                        "Rainbow",
                                    );
                                    ui.selectable_value(
                                        &mut self.strips[i]
                                            .mode_cfg
                                            .lock()
                                            .unwrap()
                                            .mode,
                                        Mode::Blink,
                                        "Blink",
                                    );
                                });
                            ui.end_row();
                        });
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Export").clicked() {
                            self.show_code = !self.show_code;
                        }
                    });
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("plot")
                .num_columns(1)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (num, s) in self.strips.iter().enumerate() {
                        Plot::new(format!("led {num}")).data_aspect(1.0).show(
                            ui,
                            |ui| {
                                for (i, c) in
                                    s.buf.lock().unwrap().iter().enumerate()
                                {
                                    let point = Points::new([i as f64, 0.0])
                                        .name("dot")
                                        .filled(true)
                                        .radius(s.radius)
                                        .shape(MarkerShape::Circle)
                                        .color(Color32::from_rgb(
                                            c.0, c.1, c.2,
                                        ));
                                    ui.points(point)
                                }
                            },
                        );
                        ui.end_row();
                    }
                });
            if !self.strips.is_empty() {
                ui.ctx().request_repaint()
            }
        });
        egui::Window::new("code")
            .open(&mut self.show_code)
            .default_size([800.0, 400.0])
            .vscroll(false)
            .hscroll(true)
            .show(ctx, |ui| {
                ui.label("example");
            });
    }
}
