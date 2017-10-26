use std::collections::BTreeMap;
use engine::entity::components;
use engine::entity::EntityId;
use super::System;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveUpdate {
    pub new_x: f64,
    pub new_y: f64,
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
    move_components: BTreeMap<EntityId, components::Movement>,
    partial_updates: BTreeMap<EntityId, MoveUpdate>,
}

impl Movement {
    pub fn new() -> Self {
        Movement {
            move_components: BTreeMap::new(),
            partial_updates: BTreeMap::new(),
        }
    }
}

impl System for Movement {
    type Update = MoveUpdate;
    type Result = Vec<MoveResult>;
    type Component = components::Movement;

    fn update(
        &mut self,
        updates: &mut [MoveUpdate],
        delta_t: f64,
        prev_result: Option<Vec<MoveResult>>,
    ) -> Vec<MoveResult> {
        for update in updates {
            // Just throw away old updates if we get new info
            self.partial_updates.insert(update.e_id, *update);
        }

        let mut results =
            prev_result.unwrap_or_else(|| Vec::with_capacity(self.partial_updates.len()));

        let mut finished_updates = Vec::new();

        for update in self.partial_updates.values_mut() {
            let pos = self.move_components.get_mut(&update.e_id).expect(&format!(
                "Movement events should only be \
                 generated for physical objects with a Pos component. Got ID: {}\n\
                 Note: ment update as a whole was {:?}",
                update.e_id,
                update
            ));

            let delta_x = (update.new_x - pos.x) / (update.new_x - pos.x) * update.speed * delta_t;
            let delta_y = (update.new_y - pos.y) / (update.new_y - pos.y) * update.speed * delta_t;

            let mut new_x = pos.x + delta_x;
            let mut new_y = pos.y + delta_y;

            // overshoot detection
            if (new_x - update.new_x).signum() != (pos.x - update.new_x).signum() {
                new_x = update.new_x;
            }

            if (new_y - update.new_y).signum() != (pos.y - update.new_y).signum() {
                new_y = update.new_y;
            }

            pos.x = new_x;
            pos.y = new_y;

            if pos.x == update.new_x && pos.y == update.new_y {
                finished_updates.push(update.e_id);
            }

            results.push(MoveResult {
                new_x: if delta_x != 0.0 { Some(pos.x) } else { None },
                new_y: if delta_y != 0.0 { Some(pos.y) } else { None },
                e_id: update.e_id,
            });
        }

        for id in finished_updates {
            self.move_components.remove(&id);
        }

        results
    }

    fn add_component(&mut self, e_id: EntityId, component: components::Movement) {
        self.move_components.insert(e_id, component);
    }

    fn remove_entity(&mut self, e_id: EntityId) {
        self.move_components.remove(&e_id);
        self.partial_updates.remove(&e_id);
    }

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component> {
        &self.move_components
    }
}

fn float_cmp(a: f64, b: f64, thresh: f64) -> bool {
    (a - b).abs() <= thresh
}
