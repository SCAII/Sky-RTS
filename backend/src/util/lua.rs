use rlua::{Error, Lua, UserData, UserDataMethods};

use engine::components::Pos;

use specs::{Entity, EntityBuilder};

#[derive(Default)]
pub struct LuaEntityBuilder {
    pos: Option<Pos>,
}

impl LuaEntityBuilder {
    pub fn into_entity<'a>(self, mut entity: EntityBuilder<'a>) -> Entity {
        if let Some(pos) = self.pos {
            entity = entity.with(pos);
        }

        entity.build()
    }
}

impl UserData for LuaEntityBuilder {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method_mut(
            "set_pos",
            |_, this, (x, y): (f64, f64)| -> Result<(), Error> {
                this.pos = Some(Pos::new(x, y));

                Ok(())
            },
        );
    }
}

// pub fn load_scaii_prelude(lua: &Lua) -> Result<(), Error> {}
