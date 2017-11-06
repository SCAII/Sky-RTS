use rlua::Lua;
use specs::System;

pub struct Init {
    _lua: Lua,
}

impl<'a> System<'a> for Init {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}
