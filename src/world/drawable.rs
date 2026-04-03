use crate::util::{rect::Rect, vec2::Vec2};

pub struct Entity {
    pub coord: Vec2, //World Coordinates
    pub size: Rect,
}

impl Entity {
    pub fn draw(&self) {}
}
