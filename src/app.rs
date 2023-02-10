use std::sync::{Arc, Mutex};

use eframe::egui::plot::{MarkerShape, Plot, Points};
use eframe::{egui, epaint::Color32};

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

pub struct App {
    led_num: Arc<Mutex<i32>>,
    radius: f32,
    show_rainbow: bool,
    data: Vec<Color>,
    rx: std::sync::mpsc::Receiver<Vec<Color>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sx, rx) = std::sync::mpsc::channel();
        let led_num = Arc::new(Mutex::new(10));
        thread_in_block(sx, led_num.clone(), cc.egui_ctx.clone());
        Self {
            radius: 25.0,
            led_num,
            show_rainbow: true,
            rx,
            data: vec![Color(0, 0, 0); 10],
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
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.heading("Config");
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("LED Num");
                    ui.add(
                        egui::DragValue::new(
                            &mut *self.led_num.lock().unwrap(),
                        )
                        .speed(1.0)
                        .clamp_range(1.0..=100.0),
                    );
                    ui.end_row();
                    ui.label("Gap");
                    ui.add(egui::DragValue::new(&mut self.radius).speed(1.0));
                    ui.end_row();
                    ui.toggle_value(&mut self.show_rainbow, "Rainbow");
                });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Ok(c) = self.rx.try_recv() {
                self.data = c;
            };

            Plot::new("main panel").data_aspect(1.0).show(ui, |ui| {
                for (i, c) in self.data.iter().enumerate() {
                    let point = Points::new([i as f64, 0.0])
                        .name("dot")
                        .filled(true)
                        .radius(self.radius)
                        .shape(MarkerShape::Circle)
                        .color(Color32::from_rgb(c.0, c.1, c.2));
                    ui.points(point)
                }
            });
        });
    }
}

fn thread_in_block(
    sx: std::sync::mpsc::Sender<Vec<Color>>,
    num: Arc<Mutex<i32>>,
    ctx: egui::Context,
) {
    let mut prev = *num.lock().unwrap();
    let mut colors = Vec::new();
    for i in 0..prev {
        colors.push(rainbow(i as f32 / prev as f32));
    }
    std::thread::spawn(move || loop {
        let curr = *num.lock().unwrap();
        if curr != prev {
            colors.clear();
            for i in 0..curr {
                colors.push(rainbow(i as f32 / curr as f32));
            }
            prev = curr;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        sx.send(colors.clone()).unwrap();
        let first = colors.remove(0);
        colors.push(first);
        ctx.request_repaint();
    });
}

fn rainbow(ratio: f32) -> Color {
    let region = (ratio * 6.0) as i32;

    let normalized: i32 = (ratio * 256.0 * 6.0).floor() as i32;
    let x = (normalized % 256) as u8;

    match region {
        0 => Color(255, x, 0),
        1 => Color(255 - x, 255, 0),
        2 => Color(0, 255, x),
        3 => Color(0, 255 - x, 255),
        4 => Color(x, 0, 255),
        5 => Color(255, 0, 255 - x),
        _ => Color(0, 0, 0),
    }
}
