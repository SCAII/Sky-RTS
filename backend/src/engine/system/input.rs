use engine::system::System;
use scaii_defs::protos::Action;
use protos::{ActionList, MoveTo};

use std::collections::BTreeMap;
use engine::entity::EntityId;
use engine::system::movement::MoveUpdate;

lazy_static!{ static ref EMPTY_MAP: BTreeMap<EntityId, ()> = { BTreeMap::new() };}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionStyle {
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
            style: ActionStyle::ActionList,
        }
    }
}

impl System for InputSystem {
    type Update = Action;
    type Result = Vec<RtsCommand>;
    type Component = ();

    fn update(
        &mut self,
        updates: &mut [Action],
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

        // using mem::replace here with a default is going to be faster than cloning the whole
        // thing due to not needing to clone the whole vec. This is essentially a move hack
        let actions = action_to_unit_actions(mem::replace(&mut updates[0], Action::default()));

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
