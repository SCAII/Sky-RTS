use engine::system::System;
use scaii_defs::protos::Action;
use protos::{ActionList, MoveTo};

use std::collections::BTreeMap;
use engine::entity::EntityId;
use engine::entity::components::Pos;
use engine::system::movement::MoveUpdate;

lazy_static!{ static ref EMPTY_MAP: BTreeMap<EntityId, ()> = { BTreeMap::new() };}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionStyle {
    Simple,
    ActionList,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum RtsCommand {
    SimpleMove(MoveUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSystem {
    style: ActionStyle,
}

impl InputSystem {
    pub fn new() -> Self {
        InputSystem {
            style: ActionStyle::Simple,
        }
    }
}

pub struct ActionInput {
    pub action: Action,
    pub positions: BTreeMap<EntityId, Pos>,
}

impl System for InputSystem {
    type Update = ActionInput;
    type Result = Vec<RtsCommand>;
    type Component = ();

    fn update(
        &mut self,
        updates: &mut [ActionInput],
        _: f64,
        result_cache: Option<Vec<RtsCommand>>,
    ) -> Vec<RtsCommand> {
        use protos::unit_action;
        use std::mem;

        let mut out = result_cache.unwrap_or_default();

        debug_assert!(updates.len() < 2);

        if updates.len() == 0 {
            return out;
        }

        match self.style {
            // using mem::replace here with a default is going to be faster than cloning the whole
            // thing due to not needing to clone the whole vec. This is essentially a move hack
            ActionStyle::ActionList => {
                let actions =
                    action_to_unit_actions(mem::replace(&mut updates[0].action, Action::default()));
                for action in actions.actions {
                    let id = action.unit_id;
                    match action.action {
                        Some(unit_action::Action::MoveTo(MoveTo { pos })) => {
                            out.push(RtsCommand::SimpleMove(MoveUpdate {
                                e_id: id as usize,
                                speed: 30.0,
                                new_x: pos.x,
                                new_y: pos.y,
                            }))
                        }
                        _ => unimplemented!(),
                    }
                }

                out
            }
            ActionStyle::Simple => {
                let positions = &updates[0].positions;
                let act = updates[0].action.discrete_actions.pop().unwrap();

                let dest = positions.get(&(act as usize)).cloned().unwrap_or(Pos {
                    x: 50.0,
                    y: 50.0,
                    heading: 0.0,
                });

                vec![
                    RtsCommand::SimpleMove(MoveUpdate {
                        e_id: 0,
                        speed: 30.0,
                        new_x: dest.x,
                        new_y: dest.y,
                    }),
                ]
            }
        }
    }

    fn add_component(&mut self, _: EntityId, _: ()) {}

    fn remove_entity(&mut self, _: EntityId) {}

    fn component_map(&self) -> &BTreeMap<EntityId, ()> {
        &*EMPTY_MAP
    }
}


fn action_to_unit_actions(action: Action) -> ActionList {
    use prost::Message;
    let msg = action
        .alternate_actions
        .expect("Only action lists are supported right now");

    if msg.len() == 0 {
        return ActionList {
            actions: Vec::new(),
        };
    }


    ActionList::decode(msg).expect("Only action lists are supported right now")
}
