use std::collections::HashSet;

pub mod components;

use self::components::ComponentType;

pub type EntityId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,

    components: HashSet<ComponentType>,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}
