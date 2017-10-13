use std::collections::HashSet;
use std::cmp::Ordering;

pub mod components;

use self::components::ComponentType;

pub type EntityId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,

    components: HashSet<ComponentType>,
}

/// We define Entity equality as having the same ID
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

/// We define the ordering as the ID, so we can sort entities by ID
/// for various purposes.
impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
