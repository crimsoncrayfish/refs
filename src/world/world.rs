use std::collections::HashMap;

use crate::{
    util::pos2::Pos2,
    world::entity::{Entity, EntityId},
};

pub struct World {
    entities: HashMap<EntityId, Entity>,
    _state: State,
    selected: Vec<EntityId>,
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
            selected: Vec::new(),
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
    pub fn clear_selected(&mut self) {
        self.selected.clear();
    }
    pub fn select_entity(&mut self, id: EntityId) {
        assert!(!self.selected.contains(&id));
        self.selected.push(id);
    }
    pub fn find_top_entity_at_pos(&self, pos: Pos2) -> Option<EntityId> {
        self.entities
            .iter()
            .filter(|(_, e)| e.contains_pos(pos))
            .max_by_key(|(_, e)| e.z_index)
            .map(|(&id, _)| id)
    }
    pub fn selected_ids(&self) -> &[EntityId] {
        &self.selected
    }
    pub fn selected_entities(&mut self) -> Vec<&mut Entity> {
        self.entities
            .iter_mut()
            .filter(|(id, _)| self.selected.contains(id))
            .map(|(_, e)| e)
            .collect()
    }
    pub fn delete_selected(&mut self) {
        for id in self.selected.iter() {
            self.entities.remove(id);
        }
        self.selected.clear();
    }
}
