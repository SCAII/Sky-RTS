use engine::entity::components::Pos;
use engine::entity::EntityId;
use engine::system::System;
use std::collections::BTreeMap;

lazy_static!{ static ref EMPTY_MAP: BTreeMap<EntityId, ()> = { BTreeMap::new() };}

pub enum VictoryState {
    Victory,
    Defeat,
    Continue,
}

pub struct TriggerInput {
    pub positions: BTreeMap<EntityId,Pos>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Trigger {}

impl Trigger {
    pub fn new() -> Self {
        Trigger{}
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
        let good_pos = updates[0].positions.get(&1).unwrap();
        let bad_pos = updates[0].positions.get(&2).unwrap();

        if pos_eq(agent_pos, good_pos) {
            VictoryState::Victory
        } else if dist(agent_pos,bad_pos) < 10.0 {
            VictoryState::Defeat
        } else {
            VictoryState::Continue
        }
    }

    fn add_component(&mut self, _: EntityId, _: Self::Component) {}

    fn remove_entity(&mut self, _: EntityId) {}

    fn component_map(&self) -> &BTreeMap<EntityId, Self::Component> {
        &*EMPTY_MAP
    }
}

#[inline]
fn pos_eq(a: &Pos, b: &Pos) -> bool {
    (a.x-b.x).abs() <= 1e-4 && (a.y-b.y).abs() <= 1e-4
}

fn dist(a: &Pos, b: &Pos) -> f64 {
    let tmp = (a.x - b.x) * (a.x - b.x) + (a.y-b.y) * (a.y-b.y);
    tmp.sqrt()
}