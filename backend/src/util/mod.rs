const SEED_SIZE: usize = 256;

use std::ops::{Deref, DerefMut};

use rand::{Isaac64Rng, Rng, SeedableRng};
use rand;
use rlua::Lua;

pub fn make_rng() -> Isaac64Rng {
    Isaac64Rng::from_seed(&seed()[..])
}

pub fn diverge(rng: &mut Isaac64Rng) {
    rng.reseed(&seed()[..]);
}

fn seed() -> [u64; SEED_SIZE] {
    let mut buf = [0; SEED_SIZE];
    for i in 0..SEED_SIZE {
        buf[i] = rand::thread_rng().gen();
    }
    buf
}

/// This is a super unsafe sendable Lua, it requires all user data to be send, which
/// we verify ourselves
pub struct SendableLua(pub Lua);

unsafe impl Send for SendableLua {}

impl Deref for SendableLua {
    type Target = Lua;

    fn deref(&self) -> &Lua {
        &self.0
    }
}

impl DerefMut for SendableLua {
    fn deref_mut(&mut self) -> &mut Lua {
        &mut self.0
    }
}
