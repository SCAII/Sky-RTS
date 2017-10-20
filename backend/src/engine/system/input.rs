use engine::system::System;
use scaii_defs::protos::Action;

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
