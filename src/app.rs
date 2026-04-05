use std::time::{Duration, SystemTime};

use eframe::egui::{self, Color32, CornerRadius, InputState, PointerButton, Pos2, Response};

use crate::{camera::Camera, draw, world::world::World};

pub struct App {
    state: AppState,
    world: World,
    camera: Camera,
}

pub struct AppState {
    mouse_pos: Option<egui::Pos2>, //Window Coordinates
    started_at: SystemTime,
    elapsed: Duration,
    last_fps: f64,
    current_fps: f64,
    drawn_entities: i32,

    pub is_insert: bool,
    pub track_fps: bool,
    pub allow_inputs: bool,
    pub debug: bool,
    pub show_coords: bool,
    pub show_grid: bool,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            started_at: SystemTime::now(),
            elapsed: Duration::from_secs(0),
            is_insert: false,
            track_fps: true,
            show_coords: false,
            show_grid: true,
            debug: false,
            allow_inputs: true,
            last_fps: 0.0,
            current_fps: 0.0,
            mouse_pos: None,
            drawn_entities: 0,
        }
    }
    pub fn reset(&mut self) {
        self.is_insert = false;
        self.show_coords = false;
        self.show_grid = true;
        self.track_fps = false;
        self.debug = false;
    }
    pub fn toggle_insert(&mut self) {
        self.is_insert = !self.is_insert;
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
    pub fn drawn_entities(&self) -> i32 {
        self.drawn_entities
    }
    pub fn set_drawn_entities(&mut self, count: i32) {
        self.drawn_entities = count;
    }
    pub fn set_mouse_pos(&mut self, pos: Option<Pos2>) {
        self.mouse_pos = pos
    }
    pub fn mouse_pos(&self) -> Option<Pos2> {
        self.mouse_pos
    }
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
    pub fn last_fps(&self) -> f64 {
        self.last_fps
    }
    pub fn calculate_fps(&mut self) {
        if !self.track_fps {
            return;
        }
        match self.started_at.elapsed() {
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
            Err(_) => (),
        };
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            camera: Camera::new(),
            world: World::new(),
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
    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::all());

        self.camera.update_coords(ui.max_rect());
        self.state
            .set_mouse_pos(ui.input(|i| i.pointer.interact_pos()));

        let rect = response.rect;
        // Background
        painter.rect_filled(rect, CornerRadius::ZERO, Color32::from_rgb(25, 25, 30));

        // Grid
        if self.state.show_grid {
            draw::draw_grid(&painter, rect, &self.camera);
        }
        if self.state.show_coords {
            draw::draw_coords(&painter, rect, &self.camera);
        }
        draw::draw_world(&painter, &self.world, &self.camera, &mut self.state);

        self.state.allow_inputs = true;
        if self.state.debug {
            self.state.calculate_fps();
            if let Some(debug_response) =
                draw::draw_debug_window(painter.ctx(), &self.state, &self.camera, &self.world)
            {
                if debug_response.contains_pointer() {
                    self.state.allow_inputs = false;
                }
            }
        };
        draw::draw_state(painter.ctx(), &self.state);
        if self.state.allow_inputs {
            self.handle_canvas_response(ui, &response);
        }
    }
    fn handle_canvas_response(&mut self, ui: &egui::Ui, response: &Response) {
        let ctrl = ui.input(|i| i.modifiers.ctrl);
        let insert = self.state.is_insert;
        if response.dragged_by(PointerButton::Middle) {
            ui.ctx()
                .output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
            self.camera.update_offset(response.drag_delta());
        }
        if insert && response.contains_pointer() {
            if ui.input(|i| i.pointer.primary_pressed()) {
                if let Some(mouse_pos) = self.state.mouse_pos() {
                    self.world
                        .add_entity_at(self.camera.pos2_to_world_pos2(mouse_pos));
                }
            }
        }
        // TODO: Some janky sht here
        if !insert && response.clicked() {
            if !ctrl {
                self.world.clear_selected();
            }
            if let Some(mouse_pos) = self.state.mouse_pos() {
                if let Some(id) = self
                    .world
                    .find_top_entity_at_pos(self.camera.pos2_to_world_pos2(mouse_pos))
                {
                    self.world.select_entity(id);
                }
            }
        }
        if !insert && response.contains_pointer() {
            if ui.input(|i| i.pointer.primary_pressed()) {
                if let Some(mouse_pos) = self.state.mouse_pos() {
                    if let Some(id) = self
                        .world
                        .find_top_entity_at_pos(self.camera.pos2_to_world_pos2(mouse_pos))
                    {
                        if !ctrl && !self.world.selected_ids().contains(&id) {
                            self.world.clear_selected();
                        }
                        self.world.select_entity(id);
                    }
                }
            }
        }
        if response.dragged_by(PointerButton::Primary) {
            self.world
                .selected_entities()
                .iter_mut()
                .for_each(|e| e.move_by(self.camera.vec2_to_world_vec2(response.drag_delta())));
        }
    }
    fn handle_global_inputs(&mut self, i: &InputState) {
        if i.raw_scroll_delta.y != 0.0 {
            self.handle_scroll(i.raw_scroll_delta.y, self.state.mouse_pos);
        }
    }
    fn handle_global_inputs_always(&mut self, i: &InputState) {
        if i.key_pressed(egui::Key::I) {
            self.state.toggle_insert();
        }
        if !i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
            self.state.toggle_grid();
        }
        if i.modifiers.ctrl && i.key_pressed(egui::Key::Space) {
            self.state.toggle_coords();
        }
        if !i.modifiers.ctrl && i.key_pressed(egui::Key::Escape) {
            self.reset();
        }
        if i.modifiers.ctrl && i.key_pressed(egui::Key::Escape) {
            self.state.toggle_debug();
        }
        if i.key_pressed(egui::Key::Delete) && self.world.selected_ids().len() > 0 {
            self.world.delete_selected();
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if self.state.allow_inputs {
                self.handle_global_inputs(i);
            }
            self.handle_global_inputs_always(i);
        });
        ctx.request_repaint();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                self.draw_canvas(ui);
            });
    }
}
