use eframe::egui::{self, Color32, CornerRadius, Stroke};

use crate::{
    camera::Camera,
    util::{pos2::Pos2, rect::Rect, vec2::Vec2},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(u32);
impl EntityId {
    pub fn new(val: u32) -> Self {
        Self(val)
    }
    pub fn next(&self) -> Self {
        Self::new(self.0 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Entity {
    pub rect: Rect,
    pub z_index: u32,
}

impl Entity {
    pub fn new_at_pos(pos: Pos2, size: Vec2) -> Self {
        Self {
            rect: Rect::new(pos - size / 2.0, size),
            z_index: 0,
        }
    }
    pub fn coord(&self) -> Pos2 {
        self.rect.pos
    }
    pub fn draw_at(&self, painter: &egui::Painter, camera: &Camera, selected: bool) {
        let radius = self.rect.width() / 2.0 * camera.zoom();
        let color = if selected {
            Color32::from_rgb(0, 0, 255)
        } else {
            Color32::from_rgb(100, 100, 100)
        };
        let pos = camera.world_pos2_to_pos2(self.rect.center());
        if selected {
            let rect = egui::Rect::from_center_size(pos, egui::vec2(radius * 2.0, radius * 2.0));
            draw_selection_box(painter, rect);
        }
        painter.circle_filled(pos, radius, color);
    }
    pub fn contains_pos(&self, pos: Pos2) -> bool {
        self.rect.contains(pos)
    }
    pub fn move_by(&mut self, delta: Vec2) {
        self.rect.pos += delta;
    }
    pub fn scale(&mut self, scale: f32) {
        self.rect.scale_current_from_center(scale);
    }
}

pub fn draw_selection_box(painter: &egui::Painter, rect: egui::Rect) {
    let selection_blue = Color32::from_rgb(0, 120, 215);
    let visual_rect = rect.expand(4.0);

    let line_stroke = Stroke::new(2.0, selection_blue);
    painter.rect_stroke(
        visual_rect,
        CornerRadius::ZERO,
        line_stroke,
        egui::StrokeKind::Outside,
    );

    let corners = [
        visual_rect.left_top(),
        visual_rect.right_top(),
        visual_rect.left_bottom(),
        visual_rect.right_bottom(),
    ];

    let handle_stroke = Stroke::new(1.0, selection_blue);
    for pos in corners {
        painter.circle_filled(pos, 4.0, Color32::WHITE);
        painter.circle_stroke(pos, 4.0, handle_stroke);
    }
}
