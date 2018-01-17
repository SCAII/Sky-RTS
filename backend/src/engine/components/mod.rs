use nalgebra::Vector2;

use specs::{Component, FlaggedStorage, NullStorage, VecStorage};

use std::ops::{Deref, DerefMut};

use scaii_defs::protos::Pos as ScaiiPos;
use scaii_defs::protos::Shape as ScaiiShape;
use scaii_defs::protos::Rect as ScaiiRect;
use scaii_defs::protos::Triangle as ScaiiTriangle;
use scaii_defs::protos::Color as ScaiiColor;

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

    pub fn to_scaii_pos(&self) -> ScaiiPos {
        ScaiiPos {
            x: Some(self.x),
            y: Some(self.y),
        }
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

#[derive(Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Hp {
    pub max_hp: f64,
    pub curr_hp: f64,
}

#[derive(Default, Component, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[component(NullStorage)]
pub struct HpChangeFlag;

#[derive(Default, Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Damage {
    pub damage: f64,
}

#[derive(Default, Component, Copy, Clone, PartialEq, Eq, Serialize, Debug, Deserialize)]
#[component(VecStorage)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_scaii_color(&self) -> ScaiiColor {
        use std::u8;

        ScaiiColor {
            r: self.r as u32,
            g: self.g as u32,
            b: self.b as u32,
            a: u8::MAX as u32,
        }
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub enum Shape {
    Triangle { base_len: f64 },
    Rect { width: f64, height: f64 },
}

impl Shape {
    pub fn to_scaii_shape(&self, id: u64) -> ScaiiShape {
        ScaiiShape {
            id: id,
            delete: false,
            rect: match *self {
                Shape::Rect {
                    ref width,
                    ref height,
                } => Some(ScaiiRect {
                    width: Some(*width),
                    height: Some(*height),
                }),
                _ => None,
            },
            triangle: match *self {
                Shape::Triangle { ref base_len } => Some(ScaiiTriangle {
                    base_len: Some(*base_len),
                }),
                _ => None,
            },
            ..ScaiiShape::default()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug, Serialize,
         Deserialize, Component)]
#[component(VecStorage)]
pub struct FactionId(pub usize);
