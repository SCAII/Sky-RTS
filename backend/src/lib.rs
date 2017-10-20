#![allow(dead_code)]

extern crate bincode;
extern crate ndarray;
#[macro_use]
extern crate prost_derive;
extern crate rand;
extern crate rlua;
extern crate scaii_defs;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod engine;
pub(crate) mod util;
pub mod protos;

use engine::Rts;

use scaii_defs::{Backend, BackendSupported, Module, SerializationStyle};
use scaii_defs::protos::{MultiMessage, ScaiiPacket};

use std::error::Error;

const SUPPORTED: BackendSupported = BackendSupported {
    serialization: SerializationStyle::NondivergingOnly,
};

pub struct Context {
    rts: Rts,
    awaiting_msgs: Vec<MultiMessage>,
}

impl Context {
    fn diverge(&mut self) {
        unimplemented!("We don't have an RNG yet so we can't reseed and diverge")
    }
}

impl Module for Context {
    fn process_msg(&mut self, _: &ScaiiPacket) -> Result<(), Box<Error>> {
        self.awaiting_msgs.push(self.rts.update());

        Ok(())
    }

    fn get_messages(&mut self) -> MultiMessage {
        use scaii_defs;

        scaii_defs::protos::merge_multi_messages(self.awaiting_msgs.drain(..).collect())
            .unwrap_or_else(|| {
                MultiMessage {
                    packets: Vec::new(),
                }
            })
    }
}

impl Backend for Context {
    fn supported_behavior(&self) -> BackendSupported {
        SUPPORTED
    }

    fn serialize(&mut self, into: Option<Vec<u8>>) -> Result<Vec<u8>, Box<Error>> {
        use std::io::BufWriter;
        use bincode;

        let buf = into.unwrap_or_else(Vec::new);
        let mut buf = BufWriter::new(buf);

        bincode::serialize_into(&mut buf, &self.rts, bincode::Infinite)?;
        Ok(buf.into_inner()?)
    }

    fn deserialize(&mut self, buf: &[u8]) -> Result<(), Box<Error>> {
        use std::io::BufReader;
        use bincode;

        let mut buf = BufReader::new(buf);

        self.rts = bincode::deserialize_from(&mut buf, bincode::Infinite)?;

        Ok(())
    }

    fn serialize_diverging(&mut self, into: Option<Vec<u8>>) -> Result<Vec<u8>, Box<Error>> {
        let out = self.serialize(into);
        self.diverge();
        out
    }

    fn deserialize_diverging(&mut self, buf: &[u8]) -> Result<(), Box<Error>> {
        let out = self.deserialize(buf);
        self.diverge();
        out
    }
}

#[no_mangle]
pub fn new() -> Box<Module> {
    Box::new(Context {
        rts: Rts::new(),
        awaiting_msgs: vec![],
    })
}

#[no_mangle]
pub fn supported_behavior() -> BackendSupported {
    SUPPORTED
}

#[no_mangle]
pub fn new_backend() -> Box<Backend> {
    Box::new(Context {
        rts: Rts::new(),
        awaiting_msgs: vec![],
    })
}
