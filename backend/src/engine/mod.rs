pub mod graphics;
pub mod components;
pub mod systems;

use scaii_defs::protos::MultiMessage;
use specs::World;

pub struct Rts {
    world: World,
}


impl Rts {
    pub fn new() -> Self {
        use util;

        let mut world = World::new();
        world.register::<self::components::Pos>();
        world.register::<self::components::Heading>();
        let rng = util::no_fail_std_rng();
        world.add_resource(rng);

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
        unimplemented!()
    }

    pub fn update(&mut self) -> MultiMessage {
        unimplemented!()
    }
}
