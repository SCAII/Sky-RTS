use engine::graphics::{Color, Shape};
use std::default::Default;

#[derive(Debug, Hash, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ComponentType {
    Pos,
    //GroundCollider,
    //AirCollider,
    //Death,
    Renderable,
}

pub type Movement = Pos;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub pos: Pos,
    pub shape: Shape,
    pub color: Color,
}

impl Default for Renderable {
    fn default() -> Self {
        Renderable {
            pos: Pos::default(),
            shape: Shape::Triangle {
                base_len: Default::default(),
            },
            color: Color::default(),
        }
    }
}
