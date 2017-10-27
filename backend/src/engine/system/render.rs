use super::MoveResult;
use super::System;
use std::collections::BTreeMap;
use engine::entity::components::Renderable;
use engine::entity::EntityId;
use scaii_defs::protos;

pub type RenderUpdate = MoveResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Render {
    draw_components: BTreeMap<EntityId, Renderable>,
}


impl Render {
    pub fn new() -> Self {
        Render { draw_components: BTreeMap::new() }
    }
}

impl System for Render {
    type Update = RenderUpdate;
    type Result = Vec<protos::Entity>;
    type Component = Renderable;

    fn update(
        &mut self,
        updates: &mut [RenderUpdate],
        _: f64,
        prev_result: Option<Vec<protos::Entity>>,
    ) -> Vec<protos::Entity> {
        let mut viz_msgs = prev_result.unwrap_or_else(Vec::new);

        for update in updates {
            let component = self.draw_components.get_mut(&update.e_id).expect(&format!(
                "Render events should only be \
                 generated for physical objects with a Renderable component. Got ID: {}\n\
                 Note: render update as a whole was {:?}",
                update.e_id,
                update
            ));

            component.pos.x = update.new_x.unwrap_or(component.pos.x);
            component.pos.y = update.new_y.unwrap_or(component.pos.y);

            viz_msgs.push(protos::Entity {
                id: update.e_id as u64,
                shapes: Vec::new(),
                delete: false,
                pos: Some(protos::Pos {
                    x: update.new_x,
                    y: update.new_y,
                }),
            });
        }

        viz_msgs
    }

    fn add_component(&mut self, e_id: EntityId, component: Self::Component) {
        self.draw_components.insert(e_id, component);
    }

    fn remove_entity(&mut self, e_id: EntityId) {
        self.draw_components.remove(&e_id);
    }

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component> {
        &self.draw_components
    }
}
