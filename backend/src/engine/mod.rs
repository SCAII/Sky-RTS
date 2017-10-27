pub mod entity;
pub mod graphics;
pub mod system;

use scaii_defs::protos::{Action, MultiMessage};
use rand::StdRng;
use std::collections::BTreeMap;
use scaii_defs::protos::ScaiiPacket;

use self::system::{InputSystem, Movement, Render};
use self::system::trigger::{Trigger, TriggerInput, VictoryState};
use self::system::movement::MoveResult;
use self::system::init::GameInit;
use self::entity::{EntityId, IdManager, PlayerId};

const SECONDS_PER_FRAME: f64 = 1.0 / 60.0;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rts {
    render_system: Render,
    pub movement_system: Movement,
    input_system: InputSystem,
    trigger_system: Trigger,
    factions: BTreeMap<EntityId, PlayerId>,
    init: GameInit,

    rng: StdRng,

    /* The result fields are just caches to avoid memory allocation
    so we don't need to serialize them */
    #[serde(skip)] move_result: Option<Vec<MoveResult>>,

    id_manager: IdManager,
}


impl Rts {
    pub fn new() -> Self {
        use util;
        Rts {
            render_system: Render::new(),
            movement_system: Movement::new(),
            input_system: InputSystem::new(),
            trigger_system: Trigger::new(),
            init: GameInit::Towers,
            factions: BTreeMap::new(),

            rng: util::no_fail_std_rng(),

            move_result: None,

            id_manager: IdManager::new(),
        }
    }

    fn build_state(&self, vic: VictoryState) -> ScaiiPacket {
        use ndarray::Array3;
        use scaii_defs::protos::scaii_packet::SpecificMsg;
        use scaii_defs::protos::endpoint::Endpoint;
        use scaii_defs::protos;
        use scaii_defs::protos::{AgentEndpoint, BackendEndpoint, State};
        use std::collections::HashMap;

        let mut tile_map = Array3::zeros((2000 / 20, 2000 / 20, 3));

        for id in self.movement_system.move_components.keys() {
            let pos = self.movement_system.move_components.get(id).unwrap();
            tile_map[((pos.x / 20.0) as usize, (pos.y / 20.0) as usize, *id)] = 1.0;
        }

        let flat_map = tile_map.into_raw_vec();

        let (reward, id) = vic.typed_reward();
        let mut reward_map = HashMap::with_capacity(1);
        reward_map.insert(format!("{}", id), reward);

        let terminal = match vic {
            VictoryState::Victory(..) | VictoryState::Defeat(..) => true,
            _ => false,
        };

        ScaiiPacket {
            src: protos::Endpoint {
                endpoint: Some(Endpoint::Backend(BackendEndpoint {})),
            },
            dest: protos::Endpoint {
                endpoint: Some(Endpoint::Agent(AgentEndpoint {})),
            },
            specific_msg: Some(SpecificMsg::State(State {
                features: flat_map,
                feature_array_dims: vec![2000 / 20, 2000 / 20, 3],
                reward: Some(reward),
                expanded_state: None,
                typed_reward: reward_map,
                terminal: terminal,
            })),
        }
    }

    pub fn restart(&mut self) -> MultiMessage {
        use scaii_defs::protos::scaii_packet::SpecificMsg;
        use scaii_defs::protos::ScaiiPacket;
        use scaii_defs::protos::endpoint::Endpoint;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, ModuleEndpoint};

        let (viz_init, viz) = self.init.init(
            &mut self.render_system,
            &mut self.movement_system,
            &mut self.factions,
            &mut self.rng,
        );

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

        let state_packet = self.build_state(VictoryState::Continue(0.0, 0));

        MultiMessage {
            packets: vec![init_packet, viz_packet, state_packet],
        }
    }

    pub fn update(&mut self, msg: Option<&Action>) -> MultiMessage {
        use self::system::System;
        use std::mem;
        use scaii_defs::protos;
        use scaii_defs::protos::{BackendEndpoint, Endpoint, ModuleEndpoint, ScaiiPacket, Viz};
        use scaii_defs::protos::scaii_packet::SpecificMsg;
        use self::system::input::RtsCommand;
        use self::system::input::ActionInput;

        let actions = match msg {
            Some(action) => vec![action.clone()],
            None => vec![],
        };

        let mut actions: Vec<_> = actions
            .into_iter()
            .map(|a| {
                ActionInput {
                    action: a,
                    positions: self.movement_system.move_components.clone(),
                }
            })
            .collect();

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
            factions: self.factions.clone(),
        };

        let vic_state = self.trigger_system
            .update(&mut vec![trigger], SECONDS_PER_FRAME, None);

        let state_packet = self.build_state(vic_state);


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
                state_packet,
            ],
        }
    }
}
