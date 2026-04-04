use crate::{util::pos2::Pos2, world::entity::Entity};

pub struct World {
    entities: Vec<Entity>,
    _state: State,
}
pub struct State {
    _exists: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            _state: State { _exists: true },
        }
    }
    pub fn add_entity_at(&mut self, pos: Pos2) {
        self.entities.push(Entity::new_at_pos(pos));
    }
    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
