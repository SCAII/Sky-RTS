use rlua::Lua;
use scaii_defs::protos::Error as ScaiiError;

use specs::System;

use util::SendableLua;

use std::marker::PhantomData;

pub(crate) mod userdata;

#[derive(SystemData)]
pub struct LuaSystemData<'a> {
    _pd: PhantomData<&'a ()>,
}

pub struct LuaSystem {
    lua: SendableLua,
}

impl LuaSystem {
    pub fn new() -> Self {
        LuaSystem {
            lua: SendableLua(Lua::new()),
        }
    }

    pub fn from_lua(lua: Lua) -> Self {
        LuaSystem {
            lua: SendableLua(lua),
        }
    }

    pub fn add_lua(&mut self, src: &str) -> Result<(), ScaiiError> {
        self.lua
            .exec::<()>(src, Some("Scenario Script File"))
            .or_else(|e| {
                Err(ScaiiError {
                    fatal: Some(true),
                    error_info: None,
                    description: format!("Cannot execute scenario description lua file:\n\t{}", e),
                })
            })
    }
}

impl<'a> System<'a> for LuaSystem {
    type SystemData = LuaSystemData<'a>;

    fn run(&mut self, _sys_data: Self::SystemData) {}
}
