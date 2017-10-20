const SEED_SIZE: usize = 20;

use rand::{Rng, SeedableRng, StdRng};
use rand;

/// Allows us to create a new StdRng, but
/// uses a fallback pseudorandom `thread_rng`
/// array of values as the seed
/// in the case where the real OS-level
/// seeding fails
pub fn no_fail_std_rng() -> StdRng {
    let rng = StdRng::new();

    match rng {
        Err(_) => StdRng::from_seed(&fallback_seed()[..]),
        Ok(rng) => rng,
    }
}

pub fn diverge_std_rng(rng: &mut StdRng) {
    *rng = no_fail_std_rng();
}

pub fn diverge_std_rng_fast(rng: &mut StdRng) {
    rng.reseed(&fallback_seed()[..]);
}

fn fallback_seed() -> [usize; SEED_SIZE] {
    rand::thread_rng().gen()
}
