const SEED_SIZE: usize = 256;

use rand::{Isaac64Rng, Rng, SeedableRng};
use rand;

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
