use rlua::{Lua, Table};
use scaii_defs::protos::Error as ScaiiError;

use specs::{System, World};

use std::marker::PhantomData;
use std::error::Error;
use std::path::Path;
use std::fmt::Debug;

pub(crate) mod userdata;

#[derive(SystemData)]
pub struct LuaSystemData<'a> {
    _pd: PhantomData<&'a ()>,
}

pub struct LuaSystem {
    lua: Lua,
}

unsafe impl Send for LuaSystem {}

impl LuaSystem {
    pub fn new() -> Self {
        LuaSystem { lua: Lua::new() }
    }

    pub fn from_lua(lua: Lua) -> Self {
        LuaSystem { lua: lua }
    }

    pub fn add_lua(&mut self, src: &str) -> Result<(), ScaiiError> {
        self.lua
            .exec::<()>(src, Some("Loading Scenario Script File"))
            .or_else(|e| {
                Err(ScaiiError {
                    fatal: Some(true),
                    error_info: None,
                    description: format!("Cannot execute scenario description lua file:\n\t{}", e),
                })
            })
    }

    pub fn reset_rng_ptr(&mut self, world: &mut World) {
        use self::userdata::UserDataRng;
        use rand::Isaac64Rng;

        let rng: *mut Isaac64Rng = &mut *world.write_resource();
        let rng = UserDataRng { rng: rng };

        self.lua
            .globals()
            .set("__sky_rts_rng", rng)
            .expect("Could not set world RNG as Lua global");
    }

    pub fn init<P: AsRef<Path> + Debug>(
        &mut self,
        world: &mut World,
        path: P,
    ) -> Result<(), Box<Error>> {
        use std::fs::File;
        use std::io::prelude::*;
        use self::userdata::UserDataRng;
        use rand::Isaac64Rng;

        let mut file = File::open(&path).or_else(|e| {
            Err(format!(
                "Could not load Lua file, is the path right?:\n\t{}",
                e
            ))
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read Lua contents");

        self.lua.exec::<()>(
            &contents,
            Some(&format!("Lua Scenario Script at path {:?}", path)),
        )?;

        let rng = &mut *world.write_resource::<Isaac64Rng>();
        let rng = UserDataRng { rng: rng };

        self.lua.globals().set("__sky_rts_rng", rng)?;

        Ok(())
    }

    pub fn reset(&mut self, world: &mut World) -> Result<(), Box<Error>> {
        use engine::components::{FactionId, Movable, Pos, Speed, Static};
        use engine::resources::{Player, UnitTypeMap};
        use specs::saveload::U64Marker;

        let units: Table = self.lua
            .eval("sky_reset(__sky_rts_rng)", Some("Restart function"))?;

        for unit in units.sequence_values::<Table>() {
            let unit = unit?;
            let template: String = unit.get("unit_type")?;

            let pos_table: Table = unit.get("pos")?;
            let pos = Pos::new(pos_table.get("x")?, pos_table.get("y")?);

            let faction: usize = unit.get("faction")?;

            let template = {
                let unit_types = world.read_resource::<UnitTypeMap>();

                unit_types
                    .tag_map
                    .get(&template)
                    .ok_or_else(|| format!("Could not get unit type template {}", template))?
                    .clone()
            };

            let color = { world.read_resource::<Vec<Player>>()[faction].color };

            let entity = world
                .create_entity()
                .with(pos)
                .with(template.shape)
                .with(color)
                .with(FactionId(faction))
                .marked::<U64Marker>();

            if template.movable {
                entity.with(Movable).with(Speed(template.speed))
            } else {
                entity.with(Static)
            }.build();
        }

        Ok(())
    }

    pub fn load_scenario(&mut self, world: &mut World) -> Result<(), Box<Error>> {
        use engine::components::{FactionId, Shape};
        use engine::resources::{Player, UnitType, UnitTypeMap};
        use engine::PLAYER_COLORS;

        let table: Table = self.lua
            .eval("sky_init()", Some("Initializing in sky_init from Lua"))?;

        let factions: usize = if table.contains_key("factions")? {
            table.get("factions").unwrap()
        } else {
            2
        };

        {
            let players = &mut *world.write_resource::<Vec<Player>>();
            for faction in 0..factions {
                players.push(Player {
                    color: PLAYER_COLORS[faction],
                    id: FactionId(faction),
                })
            }
        }

        {
            let unit_types: Table = table.get("unit_types")?;

            let u_type_map = &mut *world.write_resource::<UnitTypeMap>();

            let default = UnitType::default();

            for unit_type in unit_types.sequence_values::<Table>() {
                let unit_type = unit_type?;

                let mut concrete = UnitType {
                    tag: if unit_type.contains_key("tag")? {
                        unit_type.get("tag")?
                    } else {
                        default.tag.clone()
                    },
                    max_hp: if unit_type.contains_key("max_hp")? {
                        unit_type.get("max_hp")?
                    } else {
                        default.max_hp
                    },
                    movable: if unit_type.contains_key("can_move")? {
                        unit_type.get("can_move")?
                    } else {
                        default.movable
                    },
                    damage_deal_reward: if unit_type.contains_key("damage_deal_reward")? {
                        unit_type.get("damage_deal_reward")?
                    } else {
                        default.damage_deal_reward
                    },
                    damage_recv_penalty: if unit_type.contains_key("damage_recv_penalty")? {
                        unit_type.get("damage_recv_penalty")?
                    } else {
                        default.damage_recv_penalty
                    },
                    shape: if unit_type.contains_key("shape")? {
                        let shape_table: Table = unit_type.get("shape")?;
                        let body: String = shape_table.get("body")?;
                        if body == "rect" {
                            Shape::Rect {
                                width: shape_table.get("width")?,
                                height: shape_table.get("height")?,
                            }
                        } else
                        /* i.e. triangle */
                        {
                            Shape::Triangle {
                                base_len: shape_table.get("base_len")?,
                            }
                        }
                    } else {
                        default.shape
                    },
                    speed: if unit_type.contains_key("speed")? {
                        unit_type.get("speed")?
                    } else {
                        default.speed
                    },
                    ..UnitType::default()
                };

                let kill_reward = unit_type.get("kill_reward")?;
                let death_penalty = unit_type.get("death_penalty")?;

                match (kill_reward, death_penalty) {
                    (Some(kr), Some(dp)) => {
                        concrete.kill_reward = kr;
                        concrete.death_penalty = dp
                    }
                    (Some(kr), None) => {
                        concrete.kill_reward = kr;
                        concrete.death_penalty = -kr
                    }
                    (None, Some(dp)) => {
                        concrete.kill_reward = -dp;
                        concrete.death_penalty = dp
                    }
                    (None, None) => {
                        concrete.kill_reward = default.kill_reward;
                        concrete.death_penalty = default.death_penalty
                    }
                }

                u_type_map.typ_vec.push(concrete.clone());
                u_type_map.tag_map.insert(concrete.tag.clone(), concrete);
            }
        }

        Ok(())
    }
}

impl<'a> System<'a> for LuaSystem {
    type SystemData = LuaSystemData<'a>;

    fn run(&mut self, _sys_data: Self::SystemData) {}
}
