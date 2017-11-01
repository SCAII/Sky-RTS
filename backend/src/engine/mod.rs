pub mod graphics;

use scaii_defs::protos::MultiMessage;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rts {}


impl Rts {
    pub fn new() -> Self {
        Rts {}
    }

    pub fn restart(&mut self) -> MultiMessage {
        unimplemented!()
    }

    pub fn update(&mut self) -> MultiMessage {
        unimplemented!()
    }
}
