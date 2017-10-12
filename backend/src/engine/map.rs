use std::default::Default;
use ndarray::{Array2, Dim};

use engine::entity::EntityId;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    /* Whether the tile can be inhabited at all */
    ground_passable: bool,
    air_passable: bool,

    /* Occupied by colliders (units or buildings) */
    ground_occupied: bool,
    air_occupied: bool,

    occupied_by: Vec<EntityId>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ground_passable: true,
            air_passable: true,

            ground_occupied: false,
            air_occupied: false,

            occupied_by: Vec::with_capacity(2),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Map {
    tiles: Array2<Tile>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map { tiles: Array2::default(Dim([width, height])) }
    }
}
