#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Rect { width: f64, height: f64 },
    Triangle { base_len: f64 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
