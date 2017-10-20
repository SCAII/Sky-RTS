mod entity;
mod graphics;
mod system;

use scaii_defs::protos::MultiMessage;

use self::system::{Movement, Render};
use self::system::movement::MoveResult;
use self::entity::IdManager;

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rts {
    render_system: Render,
    movement_system: Movement,

    /* The result fields are just caches to avoid memory allocation
    so we don't need to serialize them */
    #[serde(skip)] move_result: Option<Vec<MoveResult>>,

    id_manager: IdManager,
}


impl Rts {
    pub fn new() -> Self {
        Rts {
            render_system: Render::new(),
            movement_system: Movement::new(),

            move_result: None,

            id_manager: IdManager::new(),
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

        let entities = self.render_system
            .update(&mut move_result, SECONDS_PER_FRAME, None);

        move_result.clear();
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
