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

#[derive(Default)]
pub struct MovedFlag;

impl Component for MovedFlag {
    type Storage = NullStorage<Self>;
}
