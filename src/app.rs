use std::time::{Duration, SystemTime};

use eframe::egui::{self, Color32, CornerRadius, Rect, Stroke, Vec2, pos2};

pub struct App {
    last_fps: f64,
    current_fps: f64,
    latest_err: String,
    elapsed: Duration,
    start: SystemTime,
    state: AppState,
}

pub struct AppState {
    track_fps: bool,
    debug: bool,
    show_coords: bool,
    show_grid: bool,
    zoom: f32,
    offset: egui::Vec2,
    mouse_pos: Option<egui::Pos2>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            last_fps: 0.0,
            current_fps: 0.0,
            latest_err: "".to_string(),
            elapsed: Duration::from_secs(0),
            start: SystemTime::now(),
            state: AppState {
                offset: egui::Vec2::ZERO,
                track_fps: true,
                debug: true,
                show_grid: true,
                show_coords: false,
                zoom: 25.0,
                mouse_pos: None,
            },
        }
    }
}

impl App {
    pub fn toggle_grid(&mut self) {
        self.state.show_grid = !self.state.show_grid;
    }
    pub fn toggle_coords(&mut self) {
        self.state.show_coords = !self.state.show_coords;
    }
    pub fn reset(&mut self) {
        self.state.offset = Vec2::ZERO;
        self.state.zoom = 25.0;
    }
    pub fn handle_scroll(&mut self, scroll_by: f32 /*, mouse_pos:f32*/) {
        //change the grid spacing between 25px and 100px to simulate zoom
        //let old_zoom = self.state.zoom;
        self.state.zoom = (self.state.zoom + scroll_by * 0.5).clamp(25.0, 100.0);

        //slef.state.offset = self.state.offset + (1/old_zoom - 1/self.state.zoom)*(mouse_pos)
    }
    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        self.state.mouse_pos = ui.input(|i| i.pointer.interact_pos());

        let rect = response.rect;
        if response.dragged_by(egui::PointerButton::Middle) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            self.state.offset += response.drag_delta()
        }

        // Background
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(25, 25, 30));

        // Grid
        if self.state.show_grid {
            self.draw_grid(&painter, rect);
        }
        if self.state.show_coords {
            self.draw_coords(&painter, rect);
        }

        if self.state.debug {
            self.draw_debug_window(&painter, rect);
        };
        if self.state.track_fps {
            match self.start.elapsed() {
                Ok(elapsed) => {
                    let elapsed_millis = elapsed - self.elapsed;
                    if elapsed_millis > Duration::from_millis(100) {
                        let secs = elapsed_millis.as_secs_f64();
                        self.last_fps = (self.current_fps / secs).floor();
                        self.current_fps = 0.0;
                        self.elapsed = elapsed;
                    }
                    self.current_fps += 1.0;
                }
                Err(e) => self.latest_err = e.to_string(),
            };
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let spacing = self.state.zoom;
        let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

        // Vertical lines
        let mut x = (self.state.offset.x % spacing + spacing) % spacing;
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
        let mut y = (self.state.offset.y % spacing + spacing) % spacing;
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
    fn draw_coords(&self, painter: &egui::Painter, rect: Rect) {
        let spacing = 100.0 * self.state.zoom / 25.0;

        // Vertical lines
        let mut x = (self.state.offset.x % spacing + spacing) % spacing - spacing;
        while x < rect.width() {
            let mut y = (self.state.offset.y % spacing + spacing) % spacing - spacing;
            while y < rect.height() {
                painter.text(
                    pos2(x, y),
                    egui::Align2::LEFT_TOP,
                    format!(
                        "({:.1}:{:.1})",
                        (x - self.state.offset.x) * 25.0 / self.state.zoom,
                        (y - self.state.offset.y) * 25.0 / self.state.zoom
                    ),
                    egui::FontId::monospace(8.0),
                    Color32::from_rgba_unmultiplied(180, 180, 180, 160),
                );

                y += spacing;
            }
            x += spacing;
        }
    }
    fn draw_debug_window(&self, painter: &egui::Painter, rect: Rect) {
        self.draw_debug_message(
            painter,
            rect,
            format!("Elapsed {}", self.elapsed.as_secs()),
            6,
        );

        if let Some(pos) = self.state.mouse_pos {
            self.draw_debug_message(
                painter,
                rect,
                format!(
                    "Mouse World pos {:.1}:{:.1}",
                    ((pos.to_vec2() - self.state.offset) * (25.0 / self.state.zoom)).x,
                    ((pos.to_vec2() - self.state.offset) * (25.0 / self.state.zoom)).y,
                ),
                5,
            );
        } else {
            self.draw_debug_message(painter, rect, "Mouse not in window".to_string(), 5);
        }
        if let Some(pos) = self.state.mouse_pos {
            self.draw_debug_message(painter, rect, format!("Mouse {:.1}:{:.1}", pos.x, pos.y), 4);
        } else {
            self.draw_debug_message(painter, rect, "Mouse not in window".to_string(), 4);
        }
        self.draw_debug_message(
            painter,
            rect,
            format!("Show grid {}", self.state.show_grid),
            3,
        );
        self.draw_debug_message(
            painter,
            rect,
            format!("Grid spacing {}", self.state.zoom),
            2,
        );
        self.draw_debug_message(
            painter,
            rect,
            format!(
                "Pan: ({:.0}, {:.0}) | Middle-drag to pan | FPS {:.0}",
                self.state.offset.x, self.state.offset.y, self.last_fps
            ),
            1,
        );
    }
    fn draw_debug_message(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        message: String,
        message_number: i32,
    ) {
        painter.text(
            pos2(
                rect.left() + 10.0,
                rect.bottom() - message_number as f32 * 25.0,
            ),
            egui::Align2::LEFT_TOP,
            message,
            egui::FontId::monospace(12.0),
            Color32::from_rgba_unmultiplied(180, 180, 180, 160),
        );
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Home) {
                self.reset();
            }
            if !i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.toggle_grid();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.toggle_coords();
            }
            let scroll = i.raw_scroll_delta.y;
            if scroll != 0.0 {
                self.handle_scroll(scroll);
            }
        });
        ctx.request_repaint();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                self.draw_canvas(ui);
            });
    }
}
