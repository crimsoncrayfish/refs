use crate::world::drawable::Entity;

pub struct World {
    drawables: Vec<Entity>,
    state: State,
}
pub struct State {
    exists: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            drawables: Vec::new(),
            state: State { exists: true },
        }
    }
}
