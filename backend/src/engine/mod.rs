pub mod components;
pub mod systems;
pub mod resources;

use self::resources::*;

use scaii_defs::protos::{Action, MultiMessage};

use specs::{Dispatcher, World};
use rlua::Lua;

use self::components::{Color, FactionId, Movable, Shape};

use std::path::PathBuf;

// 60FPS emulation since we're not
// actually measuring time elapsed
const SIXTY_FPS: f64 = 1.0 / 60.0;

lazy_static! {
    static ref PLAYER_COLORS: Vec<Color> = vec![
        Color { r: 0, g: 255, b: 0 },
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 0, b: 255 },
    ];
}

pub struct Rts<'a, 'b> {
    world: World,
    lua: Lua,
    pub lua_path: Option<PathBuf>,
    pub initialized: bool,

    sim_systems: Dispatcher<'a, 'b>,
    out_systems: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Rts<'a, 'b> {
    pub fn new() -> Self {
        use specs::DispatcherBuilder;
        use specs::saveload::{U64Marker, U64MarkerAllocator};
        use self::systems::movement::MoveSystem;
        use self::systems::input::InputSystem;
        use self::systems::proto_render::RenderSystem;
        use util;

        let mut world = World::new();
        world.register::<self::components::Pos>();
        world.register::<self::components::Heading>();
        world.register::<self::components::Move>();
        world.register::<self::components::Movable>();
        world.register::<self::components::MovedFlag>();
        world.register::<self::components::Hp>();
        world.register::<self::components::Damage>();
        world.register::<self::components::Shape>();
        world.register::<self::components::Color>();
        world.register::<self::components::Speed>();
        world.register::<U64Marker>();
        world.register::<FactionId>();

        let rng = util::make_rng();
        world.add_resource(rng);
        world.add_resource(Episode(0));
        world.add_resource(Terminal(false));
        world.add_resource(DeltaT(SIXTY_FPS));
        world.add_resource(Render::default());
        world.add_resource(NeedsKeyInfo(true));
        world.add_resource::<Vec<Player>>(Vec::new());
        world.add_resource(UnitTypeMap::default());
        world.add_resource(U64MarkerAllocator::new());
        world.add_resource(ActionInput::default());

        let simulation_builder: Dispatcher = DispatcherBuilder::new()
            .add(InputSystem::new(), "input", &[])
            .add(MoveSystem::new(), "movement", &["input"])
            .build();

        let output_builder = DispatcherBuilder::new()
            .add(RenderSystem {}, "render", &[])
            .build();

        let lua = Lua::new();

        Rts {
            world,
            lua,
            lua_path: None,
            initialized: false,
            sim_systems: simulation_builder,
            out_systems: output_builder,
        }
    }

    /// Causes the random number state to diverge
    /// so that if, say, the RTS had previously been
    /// serialized at this state before calling this function,
    /// identical inputs will cause different behavior.
    pub fn diverge(&mut self) {
        use rand::Isaac64Rng;
        use util;

        let rng = &mut *self.world.write_resource::<Isaac64Rng>();
        util::diverge(rng);
    }

    fn init_lua(&mut self) {
        use std::fs::File;
        use std::io::prelude::*;
        use rand::Isaac64Rng;
        use self::systems::lua::userdata::UserDataRng;

        let lua_path = self.lua_path
            .as_ref()
            .expect("Need a Lua file to run properly");

        let mut file = File::open(lua_path).expect("Could not open Lua file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read Lua contents");

        self.lua
            .exec::<()>(
                &contents,
                Some(&format!("Lua Scenario Script at path {:?}", lua_path)),
            )
            .expect("Could not execute user lua");

        let rng = &mut *self.world.write_resource::<Isaac64Rng>();
        let rng = UserDataRng { rng: rng };

        self.lua
            .globals()
            .set("__sky_rts_rng", rng)
            .expect("Could not set world RNG as Lua global");
    }

    fn init(&mut self) {
        use rlua::Table;

        self.init_lua();

        let table: Table = self.lua
            .eval("sky_init()", Some("Initializing in sky_init from Lua"))
            .expect("sky_init did not yield a valid table");

        let factions: usize = if table.contains_key("factions").unwrap() {
            table.get("factions").unwrap()
        } else {
            2
        };

        {
            let players = &mut *self.world.write_resource::<Vec<Player>>();
            for faction in 0..factions {
                players.push(Player {
                    color: PLAYER_COLORS[faction],
                    id: FactionId(faction),
                })
            }
        }

        {
            let unit_types: Table = table.get("unit_types").unwrap();

            let u_type_map = &mut *self.world.write_resource::<UnitTypeMap>();

            let default = UnitType::default();

            for unit_type in unit_types.sequence_values::<Table>() {
                let unit_type = unit_type.unwrap();

                let mut concrete = UnitType {
                    tag: if unit_type.contains_key("tag").unwrap() {
                        unit_type.get("tag").unwrap()
                    } else {
                        default.tag.clone()
                    },
                    max_hp: if unit_type.contains_key("max_hp").unwrap() {
                        unit_type.get("max_hp").unwrap()
                    } else {
                        default.max_hp
                    },
                    movable: if unit_type.contains_key("can_move").unwrap() {
                        unit_type.get("can_move").unwrap()
                    } else {
                        default.movable
                    },
                    damage_deal_reward: if unit_type.contains_key("damage_deal_reward").unwrap() {
                        unit_type.get("damage_deal_reward").unwrap()
                    } else {
                        default.damage_deal_reward
                    },
                    damage_recv_penalty: if unit_type.contains_key("damage_recv_penalty").unwrap() {
                        unit_type.get("damage_recv_penalty").unwrap()
                    } else {
                        default.damage_recv_penalty
                    },
                    shape: if unit_type.contains_key("shape").unwrap() {
                        let shape_table: Table = unit_type.get("shape").unwrap();
                        let body: String = shape_table.get("body").unwrap();
                        if body == "rect" {
                            Shape::Rect {
                                width: shape_table.get("width").unwrap(),
                                height: shape_table.get("height").unwrap(),
                            }
                        } else
                        /* i.e. triangle */
                        {
                            Shape::Triangle {
                                base_len: shape_table.get("base_len").unwrap(),
                            }
                        }
                    } else {
                        default.shape
                    },
                    speed: if unit_type.contains_key("speed").unwrap() {
                        unit_type.get("speed").unwrap()
                    } else {
                        default.speed
                    },
                    ..UnitType::default()
                };

                let kill_reward = unit_type.get("kill_reward").ok();
                let death_penalty = unit_type.get("death_penalty").ok();

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
    }

    pub fn reset(&mut self) -> MultiMessage {
        use rand::Isaac64Rng;
        use rlua::Table;
        use self::components::{Pos, Speed};
        use util;
        use scaii_defs::protos::ScaiiPacket;
        use scaii_defs::protos;
        use specs::saveload::U64Marker;

        if !self.initialized {
            self.init();
            self.initialized = true;
        }

        self.world.delete_all();
        // Do a fast reseed so it doesn't start looping the RNG state
        // after too many episodes
        {
            let rng = &mut *self.world.write_resource::<Isaac64Rng>();
            util::diverge(rng);

            self.world.write_resource::<Episode>().0 += 1;
            self.world.write_resource::<Terminal>().0 = false;
        }

        // Run our Lua and build all our units

        let units: Table = self.lua
            .eval("sky_reset(__sky_rts_rng)", Some("Restart function"))
            .unwrap();

        for unit in units.sequence_values::<Table>() {
            let unit = unit.unwrap();
            let template: String = unit.get("unit_type").unwrap();

            let pos_table: Table = unit.get("pos").unwrap();
            let pos = Pos::new(pos_table.get("x").unwrap(), pos_table.get("y").unwrap());

            let faction: usize = unit.get("faction").unwrap();

            let template = {
                let unit_types = self.world.read_resource::<UnitTypeMap>();

                unit_types.tag_map.get(&template).unwrap().clone()
            };

            let color = { self.world.read_resource::<Vec<Player>>()[faction].color };

            let entity = self.world
                .create_entity()
                .with(pos)
                .with(template.shape)
                .with(color)
                .with(FactionId(faction))
                .marked::<U64Marker>();

            if template.movable {
                entity.with(Movable).with(Speed(template.speed))
            } else {
                entity
            }.build();
        }

        // Ensure changes and render
        self.world.maintain();
        self.out_systems.dispatch(&self.world.res);

        // Build output (VizInit for clearing the screen; Viz for initial display)
        let viz_packet = self.world.read_resource::<Render>().0.clone();

        let scaii_packet = ScaiiPacket {
            src: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Backend(
                    protos::BackendEndpoint {},
                )),
            },
            dest: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Module(protos::ModuleEndpoint {
                    name: "viz".to_string(),
                })),
            },
            specific_msg: Some(protos::scaii_packet::SpecificMsg::VizInit(
                protos::VizInit::default(),
            )),
        };

        let mut mm = MultiMessage {
            packets: Vec::with_capacity(2),
        };

        mm.packets.push(scaii_packet);

        let scaii_packet = ScaiiPacket {
            src: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Backend(
                    protos::BackendEndpoint {},
                )),
            },
            dest: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Module(protos::ModuleEndpoint {
                    name: "viz".to_string(),
                })),
            },
            specific_msg: Some(protos::scaii_packet::SpecificMsg::Viz(viz_packet)),
        };

        mm.packets.push(scaii_packet);

        mm
    }

    pub fn update(&mut self) -> MultiMessage {
        use scaii_defs::protos;
        use scaii_defs::protos::ScaiiPacket;

        if self.world.read_resource::<Terminal>().0 {
            return Default::default();
        }

        self.sim_systems.dispatch(&self.world.res);
        self.out_systems.dispatch(&self.world.res);

        self.world.maintain();

        let scaii_packet = ScaiiPacket {
            src: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Backend(
                    protos::BackendEndpoint {},
                )),
            },
            dest: protos::Endpoint {
                endpoint: Some(protos::endpoint::Endpoint::Module(protos::ModuleEndpoint {
                    name: "viz".to_string(),
                })),
            },
            specific_msg: Some(protos::scaii_packet::SpecificMsg::Viz(
                self.world.read_resource::<Render>().0.clone(),
            )),
        };

        MultiMessage {
            packets: vec![scaii_packet],
        }
    }

    pub fn action_input(&mut self, action: Action) {
        self.world.write_resource::<ActionInput>().0 = Some(action);
    }
}

#[cfg(test)]
mod tests {
    use super::{Player, Rts};
    use std::path::PathBuf;

    #[test]
    fn start_rts() {
        let mut rts = Rts::new();
        rts.lua_path = Some(PathBuf::from(format!(
            "{}/lua/example.lua",
            env!("CARGO_MANIFEST_DIR")
        )));

        rts.init();

        assert!(rts.world.read_resource::<Vec<Player>>().len() == 2);

        let mm = rts.reset();
    }
}
