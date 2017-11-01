pub mod graphics;
pub mod components;
pub mod systems;

use scaii_defs::protos::MultiMessage;
use specs::World;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Episode(usize);
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Terminal(bool);

pub struct Rts {
    world: World,
}


impl Rts {
    pub fn new() -> Self {
        use util;
        use rlua::Lua;

        let mut world = World::new();
        world.register::<self::components::Pos>();
        world.register::<self::components::Heading>();
        world.register::<self::components::Move>();

        let rng = util::no_fail_std_rng();
        world.add_resource(rng);
        world.add_resource(Episode(0));
        world.add_resource(Terminal(false));

        Rts { world }
    }

    /// Causes the random number state to diverge
    /// so that if, say, the RTS had previously been
    /// serialized at this state before calling this function,
    /// identical inputs will cause different behavior.
    pub fn diverge(&mut self) {
        use rand::StdRng;
        use util;

        let rng = &mut *self.world.write_resource::<StdRng>();
        util::diverge_std_rng(rng);
    }

    pub fn restart(&mut self) -> MultiMessage {
        use rand::StdRng;
        use util;

        self.world.delete_all();
        // Do a fast reseed so it doesn't start looping the RNG state
        // after too many episodes
        {
            let rng = &mut *self.world.write_resource::<StdRng>();
            util::diverge_std_rng_fast(rng);

            self.world.write_resource::<Episode>().0 += 1;
            self.world.write_resource::<Terminal>().0 = false;
        }

        self.world.maintain();

        unimplemented!()
    }

    pub fn update(&mut self) -> MultiMessage {
        if self.world.read_resource::<Terminal>().0 {
            return Default::default();
        }

        self.world.maintain();
        unimplemented!()
    }
}
