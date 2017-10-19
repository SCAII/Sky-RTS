mod entity;
mod graphics;
mod system;

use scaii_defs::protos::MultiMessage;

use self::system::{Movement, Render};

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

    pub fn update(&mut self) -> MultiMessage {
        use self::system::System;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, Endpoint, ModuleEndpoint, ScaiiPacket, Viz};
        use scaii_defs::protos::scaii_packet::SpecificMsg;


        let render_updates = self.movement_system.update(Vec::new(), SECONDS_PER_FRAME);

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
                        endpoint: Some(protos::endpoint::Endpoint::Module(
                            ModuleEndpoint { name: "viz".to_string() },
                        )),
                    },
                    specific_msg: Some(SpecificMsg::Viz(packet)),
                },
            ],
        }
    }
}
