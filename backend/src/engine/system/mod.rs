use std::collections::BTreeMap;
use engine::entity::EntityId;

pub mod movement;
pub mod render;

pub use self::movement::*;
pub use self::render::*;

pub trait System {
    type Update;
    type Result;
    type Component;

    fn update(
        &mut self,
        updates: &mut [Self::Update],
        delta_t: f64,
        result_cache: Option<Self::Result>,
    ) -> Self::Result;

    fn add_component(&mut self, e_id: EntityId, component: Self::Component);

    fn remove_entity(&mut self, e_id: EntityId);

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component>;
}
