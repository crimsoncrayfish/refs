use eframe::egui;

use crate::util::Vec2;

mod util;
fn main() {
    //let options = eframe::NativeOptions::default();
    /*eframe::run_native(
        "CrayfishRefs",
        options,
        Box::new(|_cc| Ok(Box::new(RefBoardApp::default()))),
    )*/
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(6.0, 8.0);
    println!("lenght of a: {}", a.length());
    println!("distance between a and b: {}", a.distance_to(b));
    println!("midpoint: {:?}", a.lerp(b, 0.5));
}

struct RefBoardApp {
    counter: i32,
    name: String,
    show_greeting: bool,
}

impl Default for RefBoardApp {
    fn default() -> Self {
        Self {
            counter: 0,
            name: String::new(),
            show_greeting: false,
        }
    }
}
impl eframe::App for RefBoardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("CrayfishRefs");
                ui.add_space(ui.available_width());
                if ui.button("x").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.collapsing("Settings", |ui| {
                ui.checkbox(&mut self.show_greeting, "Show greeting");
                ui.horizontal(|ui| {
                    ui.label("Counter start:");
                    ui.add(egui::DragValue::new(&mut self.counter));
                });
            });
            //Counter

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });
            if ui.button("Greet").clicked() {
                self.show_greeting = true;
            }
            if self.show_greeting {
                if !self.name.is_empty() {
                    ui.label(format!("Hellow {}", self.name));
                } else {
                    ui.label(format!("Hellow anon"));
                }
            }
        });
    }
}
