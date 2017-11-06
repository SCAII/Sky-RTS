use nalgebra::Vector2;

use specs::{Component, FlaggedStorage, NullStorage, VecStorage};

use std::ops::{Deref, DerefMut};

// `move` is a reserved keyword, so we need to
// extend the name a little. Other submods should probably
// just be named things like `render` rather than
// `render_component`.
mod move_component;

pub use self::move_component::*;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pos(Vector2<f64>);

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Pos(Vector2::new(x, y))
    }
}

impl Component for Pos {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Deref for Pos {
    type Target = Vector2<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Heading(f64);

#[derive(Default, Component, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[component(NullStorage)]
pub struct MovedFlag;

#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hp {
    pub max_hp: f64,
    pub curr_hp: f64,
}

impl Component for Hp {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Damage {
    pub damage: f64,
}

#[derive(Default, Component, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub enum Shape {
    Triangle { base_len: f64 },
}
