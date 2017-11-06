use rlua::{Error, Lua, Table, UserData, UserDataMethods};

use engine::components::{Color, Damage, Hp, Pos, Shape};

use specs::{World};

#[derive(Default)]
pub struct GameBuilder {
    pub players: Table,
    pub units: Table,
}

impl GameBuilder {
    pub fn into_entities<'a>(self, world: &mut World) {}
}

impl UserData for GameBuilder {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method_mut(
            "create_unit",
            |_, this, table: Table| -> Result<(), Error> {
                this.units.push(table);
            },
        );

        methods.add_method_mut("set_players"
        |_, this, table: Table| {
            this.players.push(table)
        })
    }
}

// pub fn load_scaii_prelude(lua: &Lua) -> Result<(), Error> {}
