use specs::{HashMapStorage, VecStorage};
use specs::Entity;

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

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveTarget {
    Ground(Pos),
    // Unit(Entity), // need serialization workaround
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveAttack {
    Attack,
    Friendly,
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(HashMapStorage)]
pub struct Move {
    pub behavior: MoveBehavior,
    pub target: MoveTarget,
    pub attacking: MoveAttack,
}
