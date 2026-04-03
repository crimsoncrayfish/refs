use std::time::{Duration, SystemTime};

use eframe::egui::{self, Color32, CornerRadius, PointerButton, Pos2, Response, Stroke, pos2};

use crate::{camera::Camera, world::world::World};

pub struct App {
    state: AppState,
    _world: World,
    camera: Camera,
}

pub struct AppState {
    started_at: SystemTime,
    elapsed: Duration,
    track_fps: bool,
    last_fps: f64,
    current_fps: f64,
    debug: bool,
    show_coords: bool,
    show_grid: bool,
    mouse_pos: Option<egui::Pos2>, //Window Coordinates
}
impl AppState {
    pub fn new() -> Self {
        Self {
            started_at: SystemTime::now(),
            elapsed: Duration::from_secs(0),
            track_fps: true,
            show_coords: false,
            show_grid: true,
            debug: false,
            last_fps: 0.0,
            current_fps: 0.0,
            mouse_pos: None,
        }
    }
    pub fn reset(&mut self) {
        self.show_coords = false;
        self.show_grid = true;
        self.track_fps = false;
        self.debug = false;
    }
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }
    pub fn toggle_coords(&mut self) {
        self.show_coords = !self.show_coords;
    }
    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            camera: Camera::new(),
            _world: World::new(),
            state: AppState::new(),
        }
    }
}
impl App {
    pub fn reset(&mut self) {
        self.camera.reset();
        self.state.reset();
    }
    pub fn handle_scroll(&mut self, scroll_by: f32, mouse_pos: Option<Pos2>) {
        self.camera.update_zoom(scroll_by, mouse_pos);
    }
    pub fn handle_drag(&mut self, ui: &mut egui::Ui, response: Response) {
        if response.dragged_by(PointerButton::Middle) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            self.camera.update_offset(response.drag_delta());
        }
    }
    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        self.camera.update_coords(ui.max_rect());
        //TODO: make a draw_world fn
        /*for drawable in self.state.objects {
            if self.state.world_coordinates.contains(drawable.coord)
                || self
                    .state
                    .world_coordinates
                    .contains(drawable.coord + drawable.size)
            {
                drawable.draw();
            }
        }*/
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
                match self.state.started_at.elapsed() {
                    Ok(elapsed) => {
                        let elapsed_millis = elapsed - self.state.elapsed;
                        if elapsed_millis > Duration::from_millis(100) {
                            let secs = elapsed_millis.as_secs_f64();
                            self.state.last_fps = (self.state.current_fps / secs).floor();
                            self.state.current_fps = 0.0;
                            self.state.elapsed = elapsed;
                        }
                        self.state.current_fps += 1.0;
                    }
                    Err(_) => (),
                };
            }
        };
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let ss = 25.0 * self.camera.zoom; //screen spacing - at 1 zoom have a spacing of 25 px
        let color = Color32::from_rgba_unmultiplied(80, 80, 90, 60);

        // Vertical lines
        let screen_offset = self.camera.screen_offset();
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
    fn draw_coords(&self, painter: &egui::Painter, rect: egui::Rect) {
        let ss = 100.0 * self.camera.zoom; //At zoom = 1.0 100px spacing

        let screen_offset = self.camera.screen_offset();
        let mut x = (screen_offset.x % ss + ss) % ss - ss;
        while x < rect.width() {
            let mut y = (screen_offset.y % ss + ss) % ss - ss;
            while y < rect.height() {
                let world_coord = self.camera.pos2_to_world_pos2(pos2(x, y));
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

                            ui.monospace(format!("Elapsed {}", self.state.elapsed.as_secs()));

                            ui.monospace(format!(
                                "World Coordinates Min = {}: Max = {}",
                                self.camera.coordinates.min(),
                                self.camera.coordinates.max()
                            ));

                            if let Some(pos) = self.state.mouse_pos {
                                let world_pos = self.camera.pos2_to_world_pos2(pos);
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

                            ui.monospace(format!("Offset {}", self.camera.offset));
                            ui.monospace(format!("Zoom factor {}", self.camera.zoom));
                            ui.monospace(format!("Show grid {}", self.state.show_grid));

                            ui.monospace(format!(
                                "Pan: ({:.0}, {:.0}) | Middle-drag to pan | FPS {:.0}",
                                self.camera.offset.x, self.camera.offset.y, self.state.last_fps
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
                self.state.toggle_debug();
            }
            if !i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.state.toggle_grid();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
                self.state.toggle_coords();
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
