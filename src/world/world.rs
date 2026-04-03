use crate::world::drawable::Entity;

pub struct World {
    _drawables: Vec<Entity>,
    _state: State,
}
pub struct State {
    _exists: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            _drawables: Vec::new(),
            _state: State { _exists: true },
        }
    }
}
