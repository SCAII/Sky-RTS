use engine::graphics::{Color, Shape};

#[derive(Debug, Hash, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ComponentType {
    Pos,
    //GroundCollider,
    //AirCollider,
    //Death,
    Renderable,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub pos: Pos,
    pub shape: Shape,
    pub color: Color,
}
