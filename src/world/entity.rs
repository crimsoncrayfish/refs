use eframe::egui::{self, Color32, CornerRadius, Stroke};

use crate::util::{pos2::Pos2, rect::Rect, vec2::Vec2};

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
    pub coord: Pos2,
    pub _size: Rect,
    pub z_index: u32,
}

impl Entity {
    pub fn new_at_pos(pos: Pos2) -> Self {
        Self {
            coord: pos,
            _size: Rect::zero(),
            z_index: 0,
        }
    }
    pub fn draw_at(&self, painter: &egui::Painter, pos: egui::Pos2, zoom: f32, selected: bool) {
        let radius = 10.0 * zoom;
        let color = if selected {
            Color32::from_rgb(0, 0, 200)
        } else {
            Color32::from_rgb(100, 100, 100)
        };
        if selected {
            let rect = egui::Rect::from_center_size(pos, egui::vec2(radius * 2.0, radius * 2.0));
            draw_selection_box(painter, rect);
        }
        painter.circle_filled(pos, radius, color);
    }
    fn to_rect(&self) -> Rect {
        Rect::from_center_size(self.coord, Vec2::splat(20.0))
    }
    pub fn contains_pos(&self, pos: Pos2) -> bool {
        self.to_rect().contains(pos)
    }
    pub fn move_by(&mut self, delta: Vec2) {
        self.coord += delta;
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
