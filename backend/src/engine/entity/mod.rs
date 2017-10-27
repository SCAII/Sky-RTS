use std::collections::HashSet;
use std::cmp::Ordering;

pub mod components;

use self::components::ComponentType;

#[allow(unknown_lints)]
#[allow(unreadable_literal)]
const JS_MAX_SAFE_INTEGER: usize = 9007199254740991;
const MAX_ID_IN_USE_GUESS: usize = 1_000;
const MAX_ID_GUESS: usize = 1_000_000;

pub type EntityId = usize;

pub type PlayerId = usize;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Keeps track of the context's next available ID, as well as any
/// reclaimed IDs. This by default
/// ranges from 0 to the max safe representable integer
/// in Javascript (for the SCAII Viz client's sake).
///
/// Note that since we're using unsigned values this only represents
/// about half of the representable IDs in Javascript (which are signed).
/// This was chosen because the initial ID being 0 is strongly intuitive
/// and good for debugging. This engine likely doesn't scale to
/// ~9 quadrillion in-use IDs anyway.
///
/// Upon hitting the max ID, the manager will attempt to start reclaiming
/// IDs that were already used, but since deleted. Failing that, it will continue
/// past this safe allowable range unless the `panic_on_exceeds_max_id`
/// field is set to `true` (off by default).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdManager {
    in_use: HashSet<EntityId>,
    reclaimed: HashSet<EntityId>,
    next_id: EntityId,
    pub reclaim_thresh: usize,
    pub panic_on_exceeds_max_id: bool,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager {
            in_use: HashSet::with_capacity(MAX_ID_IN_USE_GUESS),
            reclaimed: HashSet::with_capacity(MAX_ID_GUESS),
            next_id: 0,
            reclaim_thresh: JS_MAX_SAFE_INTEGER,
            panic_on_exceeds_max_id: false,
        }
    }

    fn next_id_simple(&mut self) -> EntityId {
        let next_id = self.next_id;
        self.next_id += 1;
        debug_assert!(self.in_use.get(&next_id).is_none());

        self.in_use.insert(next_id);

        next_id
    }

    pub fn next_id(&mut self) -> EntityId {
        if self.next_id < self.reclaim_thresh {
            return self.next_id_simple();
        }

        match self.reclaimed.difference(&self.in_use).next() {
            None => {
                if self.panic_on_exceeds_max_id {
                    panic!("Exceeded maximum safe ID");
                } else {
                    self.next_id_simple()
                }
            }
            Some(&id) => {
                // Don't need to remove from reclaimed
                // because the set difference takes care of duplicates
                // for us.
                self.in_use.insert(id);
                debug_assert!(self.in_use.get(&id).is_none());

                id
            }
        }
    }

    pub fn remove_id(&mut self, id: EntityId) {
        debug_assert!(self.in_use.get(&id).is_some());

        self.reclaimed.insert(id);
        self.in_use.remove(&id);
    }
}
