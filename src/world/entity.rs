use crate::util::{pos2::Pos2, rect::Rect};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Entity {
    pub coord: Pos2, //World Coordinates
    pub _size: Rect,
}

impl Entity {
    pub fn new_at_pos(pos: Pos2) -> Self {
        Self {
            coord: pos,
            _size: Rect::zero(),
        }
    }
}
