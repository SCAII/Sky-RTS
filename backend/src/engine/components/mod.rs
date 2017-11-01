use specs::{HashMapStorage, VecStorage};
use specs::Entity;
use specs::saveload::{SaveLoadComponent, U64Marker};


use std::error::Error;
use std::fmt::Display;
use std::fmt;

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Heading(f64);

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveBehavior {
    Straight,
    Arrive,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveTarget {
    Ground(Pos),
    Unit(Entity), // need serialization workaround
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveAttack {
    Attack,
    Friendly,
}

#[derive(Component, Copy, Clone, PartialEq)]
#[component(HashMapStorage)]
pub struct Move {
    pub behavior: MoveBehavior,
    pub target: MoveTarget,
    pub attacking: MoveAttack,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkedMoveTarget {
    Ground(Pos),
    Unit(U64Marker),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveData {
    pub behavior: MoveBehavior,
    pub attacking: MoveAttack,
    pub target: MarkedMoveTarget,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NoTargetError {
    Entity(Entity),
    Marker(U64Marker),
}

impl Display for NoTargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Target not found: {:?}", self)
    }
}

impl Error for NoTargetError {
    fn description(&self) -> &str {
        "Could not find target when (de)serializing"
    }
}

impl SaveLoadComponent<U64Marker> for Move {
    type Data = MoveData;
    type Error = NoTargetError;

    fn save<F>(&self, mut ids: F) -> Result<MoveData, Self::Error>
    where
        F: FnMut(Entity) -> Option<U64Marker>,
    {
        Ok(MoveData {
            behavior: self.behavior,
            attacking: self.attacking,
            target: match self.target {
                MoveTarget::Ground(pos) => MarkedMoveTarget::Ground(pos),
                MoveTarget::Unit(entity) => MarkedMoveTarget::Unit(
                    ids(entity).ok_or_else(|| NoTargetError::Entity(entity))?,
                ),
            },
        })
    }

    fn load<F>(data: MoveData, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(U64Marker) -> Option<Entity>,
    {
        Ok(Move {
            behavior: data.behavior,
            attacking: data.attacking,
            target: match data.target {
                MarkedMoveTarget::Ground(pos) => MoveTarget::Ground(pos),
                MarkedMoveTarget::Unit(mark) => {
                    MoveTarget::Unit(ids(mark).ok_or_else(|| NoTargetError::Marker(mark))?)
                }
            },
        })
    }
}
