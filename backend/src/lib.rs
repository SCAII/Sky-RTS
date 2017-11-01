extern crate bincode;
extern crate bytes;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate rand;
extern crate rlua;
extern crate scaii_defs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate specs;
#[macro_use]
extern crate specs_derive;

pub mod engine;
pub(crate) mod util;
pub mod protos;

use engine::Rts;

use scaii_defs::{Backend, BackendSupported, Module, SerializationStyle};
use scaii_defs::protos::{MultiMessage, ScaiiPacket};

use std::error::Error;

const SUPPORTED: BackendSupported = BackendSupported {
    serialization: SerializationStyle::None,
};

pub struct Context {
    _rts: Rts,
    _awaiting_msgs: Vec<MultiMessage>,
}

impl Context {
    fn diverge(&mut self) {
        unimplemented!("We don't have an RNG yet so we can't reseed and diverge")
    }
}

impl Module for Context {
    fn process_msg(&mut self, _packet: &ScaiiPacket) -> Result<(), Box<Error>> {
        unimplemented!()
    }

    fn get_messages(&mut self) -> MultiMessage {
        unimplemented!()
    }
}

impl Backend for Context {
    fn supported_behavior(&self) -> BackendSupported {
        SUPPORTED
    }

    fn serialize(&mut self, _into: Option<Vec<u8>>) -> Result<Vec<u8>, Box<Error>> {
        unimplemented!()
    }

    fn deserialize(&mut self, _buf: &[u8]) -> Result<(), Box<Error>> {
        unimplemented!()
    }

    fn serialize_diverging(&mut self, _into: Option<Vec<u8>>) -> Result<Vec<u8>, Box<Error>> {
        unimplemented!()
    }

    fn deserialize_diverging(&mut self, _buf: &[u8]) -> Result<(), Box<Error>> {
        self.diverge();
        unimplemented!()
    }
}

#[no_mangle]
pub fn new() -> Box<Module> {
    Box::new(Context {
        _rts: Rts::new(),
        _awaiting_msgs: vec![],
    })
}

#[no_mangle]
pub fn supported_behavior() -> BackendSupported {
    SUPPORTED
}

#[no_mangle]
pub fn new_backend() -> Box<Backend> {
    Box::new(Context {
        _rts: Rts::new(),
        _awaiting_msgs: vec![],
    })
}
