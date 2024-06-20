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

pub(crate) struct Store {
    pub(crate) entities: Entities,
    prune_index: PruneIndex
}

#[derive(Clone)]
pub(crate) struct SynchronizedStore {
    pub(crate) store: Arc<RwLock<Store>>
}

impl Store {
     fn new() -> Self {
        Store {
            entities: HashMap::new(),
            prune_index: BTreeMap::new(),
        }
    }
}

impl SynchronizedStore {
    pub(crate) fn new() -> Self {
        SynchronizedStore {
            store: Arc::new(RwLock::new(Store::new())),
        }
    }
}
