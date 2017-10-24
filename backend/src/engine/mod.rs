pub mod entity;
pub mod graphics;
pub mod system;

use scaii_defs::protos::{MultiMessage,Action};

use self::system::{Movement, Render, InputSystem};
use self::system::trigger::{Trigger, VictoryState, TriggerInput};
use self::system::movement::MoveResult;
use self::entity::IdManager;

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rts {
    render_system: Render,
    pub movement_system: Movement,
    input_system: InputSystem,
    trigger_system: Trigger,

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

            move_result: None,

            id_manager: IdManager::new(),
        }
    }

    pub fn two_setup(&mut self) {
        use self::entity::components::{Pos, Renderable};
        use self::graphics::{Shape, Color};
        use engine::system::System;

        //agent
        self.movement_system.add_component(0, Pos {
            x: 50.0,
            y: 50.0,
            heading: 0.0,
        });

        self.render_system.add_component(0, Renderable {
            pos: Pos { x: 50.0, y: 50.0, heading: 0.0 },
            color: Color { r: 0, b: 255, g: 0, a: 255},
            shape: Shape::Triangle{base_len: 10.0},
        });

        //good_tower
        self.movement_system.add_component(1, Pos {
            x: 5.0,
            y: 50.0,
            heading: 0.0,
        });

        self.render_system.add_component(1, Renderable {
            pos: Pos { x: 5.0, y: 50.0, heading: 0.0 },
            color: Color { r: 0, b: 0, g: 255, a: 255},
            shape: Shape::Rect{width: 10.0, height: 10.0},
        });

        // bad
        self.movement_system.add_component(2, Pos {
            x: 95.0,
            y: 50.0,
            heading: 0.0,
        });

        self.render_system.add_component(2, Renderable {
            pos: Pos { x: 95.0, y: 50.0, heading: 0.0 },
            color: Color { r: 255, b: 0, g: 0, a: 255},
            shape: Shape::Rect{width: 10.0, height: 10.0},
        });
    }

    pub fn update(&mut self, msg: Option<&Action>) -> (MultiMessage,VictoryState) {
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

        let input_result = self.input_system.update(&mut actions, SECONDS_PER_FRAME, None);

        // TODO: change this
        let mut move_input: Vec<_> = input_result.into_iter().map(|r| match r {
            RtsCommand::SimpleMove(update) => update
        }).collect();

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
            positions: self.movement_system.component_map().clone()
        };

        let vic_state = self.trigger_system.update(&mut vec![trigger], SECONDS_PER_FRAME, None);

        (MultiMessage {
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
        }, vic_state)
    }
}
