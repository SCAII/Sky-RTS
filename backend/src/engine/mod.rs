mod entity;
mod graphics;
mod system;

use scaii_defs::protos::MultiMessage;
use scaii_defs::protos;

use self::system::{Movement, Render};
use self::system::movement::MoveResult;

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

pub struct Rts {
    render_system: Render,
    movement_system: Movement,

    render_result: Option<Vec<protos::Entity>>,
    move_result: Option<Vec<MoveResult>>,
}


impl Rts {
    pub fn new() -> Self {
        Rts {
            render_system: Render::new(),
            movement_system: Movement::new(),

            render_result: None,
            move_result: None,
        }
    }

    pub fn update(&mut self) -> MultiMessage {
        use self::system::System;
        use std::mem;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, Endpoint, ModuleEndpoint, ScaiiPacket, Viz};
        use scaii_defs::protos::scaii_packet::SpecificMsg;


        let mut move_result = self.movement_system.update(
            &mut [],
            SECONDS_PER_FRAME,
            mem::replace(&mut self.move_result, None),
        );

        let entities = self.render_system.update(
            &mut move_result,
            SECONDS_PER_FRAME,
            mem::replace(&mut self.render_result, None),
        );

        self.render_result = Some(entities.clone());
        self.move_result = Some(move_result);

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
