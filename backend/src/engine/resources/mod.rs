use std::collections::HashMap;

use super::FactionId;
use super::components::Shape;

use scaii_defs::protos::{Action, Viz};

use specs::World;

pub mod collision;

pub use self::collision::*;

// Recommended by ncollide
const COLLISION_MARGIN: f64 = 0.02;
use super::SIXTY_FPS;

pub(super) fn register_world_resources(world: &mut World) {
    use util;
    use specs::saveload::U64MarkerAllocator;
    use ncollide::world::CollisionWorld;
    use nalgebra::{Isometry2, Point2};

    let rng = util::make_rng();
    world.add_resource(rng);
    world.add_resource(Episode(0));
    world.add_resource(Terminal(false));
    world.add_resource(DeltaT(SIXTY_FPS));
    world.add_resource(Render::default());
    world.add_resource(NeedsKeyInfo(true));
    world.add_resource::<Vec<Player>>(Vec::new());
    world.add_resource(UnitTypeMap::default());
    world.add_resource(U64MarkerAllocator::new());
    world.add_resource(ActionInput::default());
    world.add_resource(CollisionWorld::<Point2<f64>, Isometry2<f64>, ()>::new(
        COLLISION_MARGIN,
    ));
}

/// The current episode, only meaningful for sequential runs.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Episode(pub usize);

/// Is this the final frame of the scenario?
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Terminal(pub bool);

/// Time since the last update, in seconds (fixed to one sixtieth of a second for our purposes).
#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeltaT(pub f64);

/// Any associated data with various game factions.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub color: super::components::Color,
    pub id: FactionId,
}

/// The output of the renderer, for use with Viz.
#[derive(Clone, PartialEq, Default)]
pub struct Render(pub Viz);

/// Tracks whether a FULL rerender (or total state, or whatever else)
/// is needed rather than a delta.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct NeedsKeyInfo(pub bool);

/// The actions coming from the Agent (or replay mechanism)
#[derive(Clone, PartialEq, Default, Debug)]
pub struct ActionInput(pub Option<Action>);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UnitType {
    pub tag: String,
    pub max_hp: usize,
    pub movable: bool,
    pub shape: Shape,
    pub kill_reward: f64,
    pub death_penalty: f64,
    pub damage_deal_reward: Option<f64>,
    pub damage_recv_penalty: Option<f64>,
    pub speed: f64,
}

impl Default for UnitType {
    fn default() -> Self {
        UnitType {
            tag: "".to_string(),
            max_hp: 100,
            movable: true,
            shape: Shape::Triangle { base_len: 10.0 },
            kill_reward: 0.0,
            death_penalty: 0.0,
            damage_deal_reward: None,
            damage_recv_penalty: None,
            speed: 20.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UnitTypeMap {
    pub typ_vec: Vec<UnitType>,
    pub tag_map: HashMap<String, UnitType>,
}
