use specs::VecStorage;

// `move` is a reserved keyword, so we need to
// extend the name a little. Other submods should probably
// just be named things like `render` rather than
// `render_component`.
mod move_component;

pub use self::move_component::*;

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Heading(f64);
