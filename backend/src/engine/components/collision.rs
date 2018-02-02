use super::Pos;

use specs::VecStorage;

#[derive(Component)]
#[component(VecStorage)]
pub struct AABB {
    pub pos: Pos,
    pub width: f64,
    pub height: f64,
}
