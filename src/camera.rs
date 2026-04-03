use eframe::egui;

use crate::util::{pos2::Pos2, rect::Rect, vec2::Vec2};

const ZOOM_SENSITIVITY: f32 = 0.005;
const MIN_ZOOM: f32 = 1.0;
const MAX_ZOOM: f32 = 5.0;
pub struct Camera {
    pub coordinates: Rect,
    pub offset: Vec2,
    pub zoom: f32,
}
impl Camera {
    pub fn new() -> Self {
        Self {
            coordinates: Rect::zero(),
            offset: Vec2::zero(),
            zoom: 1.0,
        }
    }
    pub fn reset(&mut self) {
        self.offset = Vec2::zero();
        self.zoom = 1.0;
    }
    pub fn update_zoom(&mut self, scroll_delta: f32, mouse_pos: Option<egui::Pos2>) {
        let new_zoom = (self.zoom + scroll_delta * ZOOM_SENSITIVITY).clamp(MIN_ZOOM, MAX_ZOOM);
        if let Some(m) = mouse_pos {
            self.offset = self.pos2_to_world_pos2(m) - (m / new_zoom).into();
        }
        self.zoom = new_zoom;
    }
    pub fn update_offset(&mut self, drag_delta: egui::Vec2) {
        self.offset -= Vec2::from(drag_delta) / self.zoom;
    }
    pub fn pos2_to_world_pos2(&self, coord: egui::Pos2) -> Pos2 {
        Into::<Pos2>::into(coord / self.zoom) + self.offset
    }
    pub fn screen_offset(&self) -> Vec2 {
        -self.offset * self.zoom
    }
    pub fn update_coords(&mut self, ui_max: egui::Rect) {
        //todo!("zoom isnt being considered here");
        self.coordinates = Rect::from_min_max(
            Into::<Vec2>::into(ui_max.min) + self.offset,
            Into::<Vec2>::into(ui_max.max) / self.zoom + self.offset,
        );
    }
}
