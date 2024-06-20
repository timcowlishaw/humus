use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::{HashMap, BTreeMap};

pub(crate) type Entity = HashMap<String, String>;

pub(crate) type Entities = HashMap<String, Entity>;

struct Key {
    entity: String,
    attribute: String
}

type PruneIndex = BTreeMap<u64, Key>;

struct StoreState {
    entities: Entities,
    prune_index: PruneIndex
}

impl StoreState {
     fn new() -> Self {
        StoreState {
            entities: HashMap::new(),
            prune_index: BTreeMap::new(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Store {
    state: Arc<RwLock<StoreState>>
}

impl Store {
    pub(crate) fn new() -> Self {
        Store {
            state: Arc::new(RwLock::new(StoreState::new())),
        }
    }

    pub(crate) fn create_entity(&mut self, key : String, entity : Entity) -> () {
        self.state.write().entities.insert(key, entity);
    }

    pub(crate) fn get_entity(self, key : String) -> Option<Entity> {
        self.state.read().entities.get(&key).cloned()
    }

    pub(crate) fn get_entities(self) -> Entities {
        self.state.read().entities.clone()
    }
}
