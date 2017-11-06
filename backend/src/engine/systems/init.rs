use specs::{Fetch, System};

use engine::LuaSrc;

#[derive(Default)]
pub struct Init;


impl<'a> System<'a> for Init {
    // We use FetchMut despite the interior mutability to ensure
    // only one system is scheduled to use the Lua at once for safety
    type SystemData = Fetch<'a, LuaSrc>;

    fn run(&mut self, _: Self::SystemData) {}
}
