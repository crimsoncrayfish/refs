use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Rounding, Stroke, Vec2, pos2, vec2};
mod util;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CrayfishRefs",
        options,
        Box::new(|_cc| Ok(Box::new(RefBoardApp::new()))),
    )
}

#[derive(Default)]
struct RefBoardApp {
    last_hovered: Option<egui::Pos2>,
    should_close: bool,
    circles: Vec<Circle>,
    current_size: egui::Pos2,
    debug: bool,
}
impl RefBoardApp {
    fn new() -> Self {
        Self {
            last_hovered: None,
            should_close: false,
            circles: Vec::new(),
            current_size: Pos2::ZERO,
            debug: true,
        }
    }
}
#[derive(Debug)]
struct Circle {
    center: egui::Pos2,
    size: f32,
}

impl eframe::App for RefBoardApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if self.should_close {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            let screen = ui.max_rect();

            painter.rect_filled(screen, CornerRadius::ZERO, Color32::from_rgb(30, 30, 40));

            let grid_color = Color32::from_rgba_unmultiplied(100, 100, 100, 50);
            let mut x = 0.0;
            while x < screen.width() {
                painter.line_segment(
                    [pos2(x, 0.0), pos2(x, screen.height())],
                    Stroke::new(1.0, grid_color),
                );
                x += 50.0;
            }
            let mut y = 0.0;
            while y < screen.height() {
                painter.line_segment(
                    [pos2(0.0, y), pos2(screen.width(), y)],
                    Stroke::new(1.0, grid_color),
                );
                y += 50.0;
            }

            if response.drag_started() {
                if let Some(_) = response.interact_pointer_pos() {
                    self.current_size = Pos2::ZERO;
                } else {
                    println!("couldnt find starting pos");
                }
            }
            if response.dragged() {
                self.current_size += response.drag_delta();
            }
            if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    painter.circle_stroke(
                        (pos - self.current_size).to_pos2().lerp(pos, 0.5),
                        self.current_size.to_vec2().length() / 2.0,
                        Stroke::new(1.0, Color32::GRAY),
                    );
                } else {
                    println!("couldnt find ending pos");
                }
            }
            if response.drag_stopped() {
                if let Some(pos) = response.interact_pointer_pos() {
                    self.circles.push(Circle {
                        center: (pos - self.current_size).to_pos2().lerp(pos, 0.5),
                        size: self.current_size.to_vec2().length() / 2.0,
                    });
                } else {
                    println!("couldnt find ending pos");
                }
            }

            for circle in &self.circles {
                painter.circle_filled(circle.center, circle.size, Color32::from_rgb(100, 150, 200));
            }
            /*if let Some(pos) = response.hover_pos() {
                painter.circle_stroke(pos, 20.0, Stroke::new(1.0, Color32::GRAY));
            }*/
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Space) {
                    self.should_close = true;
                }
            });
            if self.debug {
                painter.text(
                    Pos2::new(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("Drag Delta: {}", self.current_size),
                    egui::FontId::proportional(20.0),
                    Color32::RED,
                );
                painter.text(
                    Pos2::new(10.0, 20.0),
                    egui::Align2::LEFT_TOP,
                    format!("Drag Size: {}", self.current_size.to_vec2().length()),
                    egui::FontId::proportional(20.0),
                    Color32::RED,
                );
            }
        });
    }
}
