use engine::Pos;

#[derive(Debug,Hash,Copy,Clone,Serialize,Deserialize,Eq,PartialEq)]
pub enum ComponentType {
    Pos,
    GroundCollider,
    AirCollider,
    Death,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PosComponent {
    pos: Pos,
}