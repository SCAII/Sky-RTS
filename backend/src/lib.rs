extern crate ndarray;
extern crate rand;
extern crate rlua;
extern crate scaii_defs;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod engine;

use engine::Rts;

use scaii_defs::{Backend, BackendSupported, Module, SerializationStyle};
use scaii_defs::protos::{MultiMessage, ScaiiPacket};

use std::error::Error;

const SUPPORTED: BackendSupported = BackendSupported {
    serialization: SerializationStyle::None,
};

struct Context {
    rts: Rts,
    awaiting_msgs: Vec<MultiMessage>,
}

impl Module for Context {
    fn process_msg(&mut self, _: &ScaiiPacket) -> Result<(), Box<Error>> {
        self.awaiting_msgs.push(self.rts.update());

        Ok(())
    }

    fn get_messages(&mut self) -> MultiMessage {
        use scaii_defs;
        use std::mem;

        scaii_defs::protos::merge_multi_messages(mem::replace(&mut self.awaiting_msgs, Vec::new()))
            .unwrap_or(MultiMessage {
                packets: Vec::new(),
            })
    }
}

impl Backend for Context {
    fn supported_behavior(&self) -> BackendSupported {
        SUPPORTED
    }
}

#[no_mangle]
pub fn new() -> Box<Module> {
    let (rts, msgs) = Rts::rand_new();
    Box::new(Context {
        rts: rts,
        awaiting_msgs: vec![msgs],
    })
}

#[no_mangle]
pub fn supported_behavior() -> BackendSupported {
    SUPPORTED
}

#[no_mangle]
pub fn new_backend() -> Box<Backend> {
    let (rts, msgs) = Rts::rand_new();
    Box::new(Context {
        rts: rts,
        awaiting_msgs: vec![msgs],
    })
}
