use engine::system::System;
use scaii_defs::protos::Action;
use protos::{ActionList, UnitAction};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionStyle {
    ActionList,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RtsCommand {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSystem {
    style: ActionStyle,
}

// impl System for InputSystem {
//     type Update = Action;
//     type Result = Vec<RtsCommand>;
//     type Component = ();

//     fn update(&mut self, updates: &mut[Action], _: f64, result_cache: Option<Vec<RtsCommand>>) {

//     }
// }


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
