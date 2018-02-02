use std::collections::HashMap;

use super::FactionId;
use super::components::Shape;

use scaii_defs::protos::{Action, Viz};

pub mod collision;

pub use self::collision::*;

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
