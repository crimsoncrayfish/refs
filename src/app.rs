use std::time::{Duration, SystemTime};

use eframe::egui::{self, Color32, CornerRadius, Rect, Stroke, pos2};

pub struct App {
    offset: egui::Vec2,
    debug: bool,
    track_fps: bool,
    last_fps: f64,
    current_fps: f64,
    latest_err: String,
    elapsed: Duration,
    start: SystemTime,
}

impl Default for App {
    fn default() -> Self {
        Self {
            offset: egui::Vec2::ZERO,
            debug: true,
            track_fps: true,
            last_fps: 0.0,
            current_fps: 0.0,
            latest_err: "".to_string(),
            elapsed: Duration::from_secs(0),
            start: SystemTime::now(),
        }
    }
}

impl App {
    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        let rect = response.rect;
        if response.dragged_by(egui::PointerButton::Middle) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            self.offset += response.drag_delta()
        }

        // Background
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(25, 25, 30));

        // Grid
        self.draw_grid(&painter, rect);

        if self.debug {
            self.draw_debug_window(&painter, rect);
        };
        if self.track_fps {
            match self.start.elapsed() {
                Ok(elapsed) => {
                    let elapsed_millis = elapsed - self.elapsed;
                    if elapsed_millis > Duration::from_millis(100) {
                        let secs = elapsed_millis.as_secs_f64();
                        self.last_fps = (self.current_fps as f64 / secs).floor() as f64;
                        self.current_fps = 0.0;
                        self.elapsed = elapsed.clone();
                    }
                    self.current_fps += 1.0;
                }
                Err(e) => self.latest_err = e.to_string(),
            };
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let spacing = 50.0;
        let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

        // Vertical lines
        let mut x = (self.offset.x % spacing + spacing) % spacing;
        while x < rect.width() {
            painter.line_segment(
                [
                    pos2(rect.left() + x, rect.top()),
                    pos2(rect.left() + x, rect.bottom()),
                ],
                Stroke::new(1.0, color),
            );
            x += spacing;
        }

        // Horizontal lines
        let mut y = (self.offset.y % spacing + spacing) % spacing;
        while y < rect.height() {
            painter.line_segment(
                [
                    pos2(rect.left(), rect.top() + y),
                    pos2(rect.right(), rect.top() + y),
                ],
                Stroke::new(1.0, color),
            );
            y += spacing;
        }
    }
    fn draw_debug_window(&self, painter: &egui::Painter, rect: Rect) {
        painter.text(
            pos2(rect.left() + 10.0, rect.bottom() - 50.0),
            egui::Align2::LEFT_TOP,
            format!("Elapsed {}", self.elapsed.as_secs()),
            egui::FontId::monospace(12.0),
            Color32::from_rgba_unmultiplied(180, 180, 180, 160),
        );
        painter.text(
            pos2(rect.left() + 10.0, rect.bottom() - 25.0),
            egui::Align2::LEFT_TOP,
            format!(
                "Pan: ({:.0}, {:.0}) | Middle-drag to pan | FPS {:.0}",
                self.offset.x, self.offset.y, self.last_fps
            ),
            egui::FontId::monospace(12.0),
            Color32::from_rgba_unmultiplied(180, 180, 180, 160),
        );
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                self.draw_canvas(ui);
            });
    }
}
