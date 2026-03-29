use std::time::{Duration, SystemTime};

use eframe::egui::{
    self, Color32, CornerRadius, PointerButton, Pos2, Rect, Response, Stroke, Vec2, pos2,
};

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
    //TODO: convert to world coords
    offset: egui::Vec2,            //World Coordinates
    mouse_pos: Option<egui::Pos2>, //Window Coordinates
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
                debug: false,
                show_grid: true,
                show_coords: false,
                zoom: 1.0,
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
    pub fn toggle_debug(&mut self) {
        self.state.debug = !self.state.debug;
    }
    pub fn reset(&mut self) {
        self.state.offset = Vec2::ZERO;
        self.state.zoom = 1.0;
        self.state.show_coords = false;
        self.state.show_grid = true;
    }
    fn coord_to_world_coord(&self, coord: Pos2) -> Pos2 {
        coord / self.state.zoom + self.state.offset
    }
    fn screen_offset(&self) -> Vec2 {
        -self.state.offset * self.state.zoom
    }
    pub fn handle_scroll(&mut self, scroll_by: f32, _mouse_pos: Option<Pos2>) {
        //change the grid spacing between 25px and 100px to simulate zoom
        let zoom_sensitivity = 0.005;
        let new_zoom = (self.state.zoom + scroll_by * zoom_sensitivity).clamp(1.0, 5.0);
        //TODO: change offset based on mouse pos
        if let Some(m) = self.state.mouse_pos {
            self.state.offset = self.coord_to_world_coord(m) - (m / new_zoom);
        }
        self.state.zoom = new_zoom;
    }
    pub fn handle_drag(&mut self, ui: &mut egui::Ui, response: Response) {
        if response.dragged_by(PointerButton::Middle) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            self.state.offset -= response.drag_delta() / self.state.zoom;
        }
    }
    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        self.state.mouse_pos = ui.input(|i| i.pointer.interact_pos());
        let rect = response.rect;

        self.handle_drag(ui, response);

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
            self.draw_debug_window(painter.ctx());
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
        };
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let ss = 25.0 * self.state.zoom; //screen spacing - at 1 zoom have a spacing of 25 px
        let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

        // Vertical lines
        let screen_offset = self.screen_offset();
        let mut x = (screen_offset.x % ss + ss) % ss;
        while x < rect.width() {
            painter.line_segment(
                [
                    pos2(rect.left() + x, rect.top()),
                    pos2(rect.left() + x, rect.bottom()),
                ],
                Stroke::new(1.0, color),
            );
            x += ss;
        }

        // Horizontal lines
        let mut y = (screen_offset.y % ss + ss) % ss;
        while y < rect.height() {
            painter.line_segment(
                [
                    pos2(rect.left(), rect.top() + y),
                    pos2(rect.right(), rect.top() + y),
                ],
                Stroke::new(1.0, color),
            );
            y += ss;
        }
    }
    fn draw_coords(&self, painter: &egui::Painter, rect: Rect) {
        let ss = 100.0 * self.state.zoom; //At zoom = 1.0 100px spacing

        let screen_offset = self.screen_offset();
        let mut x = (screen_offset.x % ss + ss) % ss - ss;
        while x < rect.width() {
            let mut y = (screen_offset.y % ss + ss) % ss - ss;
            while y < rect.height() {
                let world_coord = self.coord_to_world_coord(pos2(x, y));
                painter.text(
                    pos2(x, y),
                    egui::Align2::LEFT_TOP,
                    format!("({:.1}:{:.1})", world_coord.x, world_coord.y),
                    egui::FontId::monospace(8.0),
                    Color32::from_rgba_unmultiplied(180, 180, 180, 160),
                );

                y += ss;
            }
            x += ss;
        }
    }
    fn draw_debug_window(&self, ctx: &egui::Context) {
        egui::Area::new(egui::Id::new("debug_info"))
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(10.0, -10.0))
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(150))
                    .corner_radius(4.0)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.visuals_mut().override_text_color =
                                Some(egui::Color32::from_gray(180));
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                            ui.monospace(format!("Elapsed {}", self.elapsed.as_secs()));

                            if let Some(pos) = self.state.mouse_pos {
                                let world_pos = self.coord_to_world_coord(pos);
                                ui.monospace(format!(
                                    "Mouse World pos {:.1}:{:.1}",
                                    world_pos.x, world_pos.y
                                ));
                            } else {
                                ui.monospace("Mouse not in window");
                            }

                            if let Some(pos) = self.state.mouse_pos {
                                ui.monospace(format!("Mouse {:.1}:{:.1}", pos.x, pos.y));
                            } else {
                                ui.monospace("Mouse not in window");
                            }

                            ui.monospace(format!("Offset {}", self.state.offset));
                            ui.monospace(format!("Show grid {}", self.state.show_grid));

                            ui.monospace(format!("Zoom factor {}", self.state.zoom));

                            ui.monospace(format!(
                                "Pan: ({:.0}, {:.0}) | Middle-drag to pan | FPS {:.0}",
                                self.state.offset.x, self.state.offset.y, self.last_fps
                            ));
                        });
                    });
            });
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if !i.modifiers.ctrl && i.key_pressed(egui::Key::Escape) {
                self.reset();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Escape) {
                self.toggle_debug();
            }
            if !i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.toggle_grid();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.toggle_coords();
            }
            let scroll = i.raw_scroll_delta.y;
            if scroll != 0.0 {
                self.handle_scroll(scroll, self.state.mouse_pos);
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
