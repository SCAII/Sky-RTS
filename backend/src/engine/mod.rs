mod entity;
mod map;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pos {
    x: usize,
    y: usize,
}
