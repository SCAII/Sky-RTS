mod entity;
mod graphics;
mod system;

use scaii_defs::protos::MultiMessage;

use self::system::{MoveUpdate, Movement, Render};

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

pub struct Rts {
    render_system: Render,
    movement_system: Movement,
}


impl Rts {
    pub fn new() -> Self {
        Rts {
            render_system: Render::new(),
            movement_system: Movement::new(),
        }
    }

    pub fn rand_new() -> (Self, MultiMessage) {
        use rand;
        use rand::Rng;
        use self::entity::components::{Pos, Renderable};
        use self::graphics::{Color, Shape};
        use self::system::System;

        use scaii_defs::protos::{BackendEndpoint, Endpoint, Entity, ModuleEndpoint, ScaiiPacket,
                                 Viz, VizInit};
        use scaii_defs::protos;
        use scaii_defs::protos::scaii_packet::SpecificMsg;

        let mut rts = Rts::new();

        let mut rng = rand::thread_rng();
        let mut entities = Vec::new();

        for id in 0..rng.gen_range(5, 11) {
            let pos = Pos {
                x: rng.gen_range(0.0, 1024.0),
                y: rng.gen_range(0.0, 1024.0),
            };

            let base_len = 10.0;
            let shape = Shape::Triangle { base_len: base_len };
            let color = Color {
                r: 255,
                g: 0,
                b: 255,
                a: 255,
            };

            let render = Renderable {
                pos: pos,
                shape: shape,
                color: color,
            };

            rts.movement_system.add_component(id, pos);
            rts.render_system.add_component(id, render);

            entities.push(Entity {
                delete: false,
                id: id as u64,
                pos: Some(protos::Pos {
                    x: Some(pos.x),
                    y: Some(pos.y),
                }),
                shapes: vec![
                    protos::Shape {
                        id: 0,
                        color: Some(protos::Color {
                            r: color.r as u32,
                            g: color.g as u32,
                            b: color.b as u32,
                            a: color.a as u32,
                        }),
                        delete: false,
                        rect: None,
                        triangle: Some(protos::Triangle {
                            base_len: Some(base_len),
                        }),
                        relative_pos: Some(protos::Pos {
                            x: Some(0.0),
                            y: Some(0.0),
                        }),
                    },
                ],
            });
        }

        let src = Endpoint {
            endpoint: Some(protos::endpoint::Endpoint::Backend(BackendEndpoint {})),
        };
        let viz_dest = Endpoint {
            endpoint: Some(protos::endpoint::Endpoint::Module(ModuleEndpoint {
                name: "viz".to_string(),
            })),
        };

        let msg = MultiMessage {
            packets: vec![
                ScaiiPacket {
                    src: src.clone(),
                    dest: viz_dest.clone(),
                    specific_msg: Some(SpecificMsg::VizInit(VizInit {})),
                },
                ScaiiPacket {
                    src: src,
                    dest: viz_dest,
                    specific_msg: Some(SpecificMsg::Viz(Viz { entities: entities })),
                },
            ],
        };

        (rts, msg)
    }

    fn rand_move_updates(&self) -> Vec<MoveUpdate> {
        use rand;
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let mut out = Vec::new();
        for id in 0..rng.gen_range(5, 10) {
            out.push(MoveUpdate {
                delta_x: rng.gen_range(0.0, 3.0),
                delta_y: rng.gen_range(0.0, 3.0),
                speed: 1.0,
                e_id: id,
            });
        }

        out
    }

    pub fn update(&mut self) -> MultiMessage {
        use self::system::System;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, Endpoint, ModuleEndpoint, ScaiiPacket, Viz};
        use scaii_defs::protos::scaii_packet::SpecificMsg;


        let move_updates = self.rand_move_updates();
        let render_updates = self.movement_system.update(move_updates, SECONDS_PER_FRAME);

        let entities = self.render_system.update(render_updates, SECONDS_PER_FRAME);

        let packet = Viz { entities: entities };

        MultiMessage {
            packets: vec![
                // Stuff sent to visualization target
                ScaiiPacket {
                    src: Endpoint {
                        endpoint: Some(protos::endpoint::Endpoint::Backend(BackendEndpoint {})),
                    },
                    dest: Endpoint {
                        endpoint: Some(protos::endpoint::Endpoint::Module(ModuleEndpoint {
                            name: "viz".to_string(),
                        })),
                    },
                    specific_msg: Some(SpecificMsg::Viz(packet)),
                },
            ],
        }
    }
}
