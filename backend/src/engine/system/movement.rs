use std::collections::BTreeMap;
use engine::entity::components::Pos;
use engine::entity::EntityId;
use super::System;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveUpdate {
    pub delta_x: f64,
    pub delta_y: f64,
    pub speed: f64,
    pub e_id: EntityId,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveResult {
    pub new_x: Option<f64>,
    pub new_y: Option<f64>,
    pub e_id: EntityId,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movement {
    pos_components: BTreeMap<EntityId, Pos>,
    partial_updates: BTreeMap<EntityId, MoveUpdate>,
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            pos_components: BTreeMap::new(),
            partial_updates: BTreeMap::new(),
        }
    }
}

impl System for Movement {
    type Update = Vec<MoveUpdate>;
    type Result = Vec<MoveResult>;
    type Component = Pos;

    fn update(&mut self, updates: Vec<MoveUpdate>, delta_t: f64) -> Vec<MoveResult> {
        for update in updates {
            // Just throw away old updates if we get new info
            self.partial_updates.insert(update.e_id, update);
        }

        let mut results = Vec::with_capacity(self.partial_updates.len());

        let mut finished_updates = Vec::new();

        for update in self.partial_updates.values_mut() {
            let pos = self.pos_components.get_mut(&update.e_id).expect(&format!(
                "Movement events should only be \
                 generated for physical objects with a Pos component. Got ID: {}\n\
                 Note: movement update as a whole was {:?}",
                update.e_id,
                update
            ));

            let delta_x = update.delta_x * delta_t;
            let delta_y = update.delta_y * delta_t;

            update.delta_x = (update.delta_x - delta_x).max(0.0);
            update.delta_y = (update.delta_y - delta_y).max(0.0);
            if update.delta_x == 0.0 && update.delta_y == 0.0 {
                finished_updates.push(update.e_id);
            }

            pos.x += delta_x;
            pos.y += delta_y;

            results.push(MoveResult {
                new_x: if delta_x == 0.0 { Some(pos.x) } else { None },
                new_y: if delta_y == 0.0 { Some(pos.y) } else { None },
                e_id: update.e_id,
            });
        }

        results
    }

    fn add_component(&mut self, e_id: EntityId, component: Pos) {
        self.pos_components.insert(e_id, component);
    }

    fn remove_entity(&mut self, e_id: EntityId) {
        self.pos_components.remove(&e_id);
        self.partial_updates.remove(&e_id);
    }

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component> {
        &self.pos_components
    }
}
