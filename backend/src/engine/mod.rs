pub mod entity;
pub mod graphics;
pub mod system;

use scaii_defs::protos::{Action, MultiMessage};

use self::system::{InputSystem, Movement, Render};
use self::system::trigger::{Trigger, TriggerInput, VictoryState};
use self::system::movement::MoveResult;
use self::system::init::GameInit;
use self::entity::IdManager;

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rts {
    render_system: Render,
    pub movement_system: Movement,
    input_system: InputSystem,
    trigger_system: Trigger,
    init: GameInit,

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
            input_system: InputSystem::new(),
            trigger_system: Trigger::new(),
            init: GameInit::Towers,

            move_result: None,

            id_manager: IdManager::new(),
        }
    }

    pub fn restart(&mut self) -> MultiMessage {
        use scaii_defs::protos::scaii_packet::SpecificMsg;
        use scaii_defs::protos::ScaiiPacket;
        use scaii_defs::protos::endpoint::Endpoint;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, ModuleEndpoint};

        let (viz_init, viz) = self.init
            .init(&mut self.render_system, &mut self.movement_system);

        let init_packet = ScaiiPacket {
            specific_msg: Some(SpecificMsg::VizInit(viz_init)),
            src: protos::Endpoint {
                endpoint: Some(Endpoint::Backend(BackendEndpoint {})),
            },
            dest: protos::Endpoint {
                endpoint: Some(Endpoint::Module(ModuleEndpoint {
                    name: "viz".to_string(),
                })),
            },
        };

        let viz_packet = ScaiiPacket {
            specific_msg: Some(SpecificMsg::Viz(viz)),
            src: init_packet.src.clone(),
            dest: init_packet.dest.clone(),
        };

        MultiMessage {
            packets: vec![init_packet, viz_packet],
        }
    }

    pub fn update(&mut self, msg: Option<&Action>) -> (MultiMessage, VictoryState) {
        use self::system::System;
        use std::mem;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, Endpoint, ModuleEndpoint, ScaiiPacket, Viz};
        use scaii_defs::protos::scaii_packet::SpecificMsg;
        use self::system::input::RtsCommand;

        let mut actions = match msg {
            Some(action) => vec![action.clone()],
            None => vec![],
        };

        let input_result = self.input_system
            .update(&mut actions, SECONDS_PER_FRAME, None);

        // TODO: change this
        let mut move_input: Vec<_> = input_result
            .into_iter()
            .map(|r| match r {
                RtsCommand::SimpleMove(update) => update,
            })
            .collect();

        let mut move_result = self.movement_system.update(
            &mut move_input,
            SECONDS_PER_FRAME,
            mem::replace(&mut self.move_result, None),
        );

        let entities = self.render_system
            .update(&mut move_result, SECONDS_PER_FRAME, None);

        move_result.clear();
        self.move_result = Some(move_result);

        let packet = Viz { entities: entities };

        let trigger = TriggerInput {
            positions: self.movement_system.component_map().clone(),
        };

        let vic_state = self.trigger_system
            .update(&mut vec![trigger], SECONDS_PER_FRAME, None);

        (
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
            },
            vic_state,
        )
    }
}
