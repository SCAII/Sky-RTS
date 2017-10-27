use engine::entity::components::Pos;
use engine::entity::{EntityId, PlayerId};
use engine::system::System;
use std::collections::BTreeMap;

lazy_static!{ static ref EMPTY_MAP: BTreeMap<EntityId, ()> = { BTreeMap::new() };}

pub enum VictoryState {
    Victory(f64, EntityId),
    Defeat(f64, EntityId),
    Continue(f64, EntityId),
}

impl VictoryState {
    pub fn typed_reward(&self) -> (f64, EntityId) {
        use self::VictoryState::{Continue, Defeat, Victory};
        match *self {
            Victory(r, id) | Defeat(r, id) | Continue(r, id) => (r, id),
        }
    }
}

pub struct TriggerInput {
    pub positions: BTreeMap<EntityId, Pos>,
    pub factions: BTreeMap<EntityId, PlayerId>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Trigger {}

impl Trigger {
    pub fn new() -> Self {
        Trigger {}
    }
}

impl System for Trigger {
    type Update = TriggerInput;
    type Result = VictoryState;
    type Component = ();

    fn update(
        &mut self,
        updates: &mut [Self::Update],
        _: f64,
        _: Option<Self::Result>,
    ) -> Self::Result {
        let agent_pos = updates[0].positions.get(&0).unwrap();

        for tower_id in updates[0].positions.keys().filter(|k| **k != 1) {
            let pos = updates[0].positions.get(tower_id).unwrap();
            let faction = updates[0].factions.get(tower_id).unwrap();
            if pos_eq(agent_pos, pos) && faction == &1 {
                return VictoryState::Victory(100.0, *tower_id);
            } else if dist(agent_pos, pos) < 50.0 && faction == &2 {
                return VictoryState::Defeat(-100.0, *tower_id);
            }
        }

        VictoryState::Continue(0.0, 0)
    }

    fn add_component(&mut self, _: EntityId, _: Self::Component) {}

    fn remove_entity(&mut self, _: EntityId) {}

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component> {
        &*EMPTY_MAP
    }
}

#[inline]
fn pos_eq(a: &Pos, b: &Pos) -> bool {
    (a.x - b.x).abs() <= 1e-4 && (a.y - b.y).abs() <= 1e-4
}

fn dist(a: &Pos, b: &Pos) -> f64 {
    let tmp = (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y);
    tmp.sqrt()
}
