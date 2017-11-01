use specs::VecStorage;

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[component(VecStorage)]
pub struct Heading(f64);
