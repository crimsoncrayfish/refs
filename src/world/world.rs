use std::collections::HashMap;

use crate::{
    util::pos2::Pos2,
    world::entity::{Entity, EntityId},
};

pub struct World {
    entities: HashMap<EntityId, Entity>,
    _state: State,
    selected: Option<EntityId>,
    next_entity_id: EntityId,
}
pub struct State {
    _exists: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            _state: State { _exists: true },
            selected: None,
            next_entity_id: EntityId::new(1),
        }
    }
    pub fn add_entity_at(&mut self, pos: Pos2) {
        self.entities
            .insert(self.next_entity_id, Entity::new_at_pos(pos));
        self.next_entity_id = self.next_entity_id.next();
    }
    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }
    pub fn select_top_entity_at_pos(&mut self, pos: Pos2) {
        let clicked = self
            .entities
            .iter()
            .filter(|(_, e)| e.contains_pos(pos))
            .max_by_key(|(_, e)| e.z_index);
        self.selected = clicked.map(|(id, _)| *id);
    }
    pub fn selected_id(&self) -> Option<EntityId> {
        self.selected
    }
    pub fn selected_entity(&mut self) -> Option<&mut Entity> {
        if let Some(selected_id) = self.selected {
            self.entities.get_mut(&selected_id)
        } else {
            None
        }
    }
    pub fn delete_entity(&mut self, entity_id: &EntityId) {
        self.entities.remove(entity_id);
    }
}
