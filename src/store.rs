use std::collections::{HashMap, BTreeMap, HashSet};
use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use basic_trie::DataTrie;
use log;
use parking_lot::RwLock;
use serde_json::{Value as JsonValue};
use uuid::Uuid;

pub(crate) type Entity = JsonValue;
type Path = String;
type Id = Uuid;
type Timestamp = u64;

struct StoreState {
    lifetime: Timestamp,
    refresh_child_entities : bool,
    entities: HashMap<Id, Entity>,
    prune_index: BTreeMap<Timestamp, HashSet<Id>>,
    reverse_prune_index: HashMap<Id, Timestamp>,
    path_index: DataTrie<Id>,
    reverse_path_index: HashMap<Id, Path>
}

impl StoreState {

     fn new() -> Self {
        StoreState {
            lifetime : env::var("HUMUS_LIFETIME").ok().and_then(|v|{ v.parse::<u64>().ok() }).unwrap_or(60u64),
            refresh_child_entities : env::var("HUMUS_REFRESH_CHILD_ENTITIES").is_ok(),
            entities: HashMap::new(),
            prune_index: BTreeMap::new(),
            reverse_prune_index: HashMap::new(),
            path_index: DataTrie::new(),
            reverse_path_index: HashMap::new(),
        }
    }

     fn insert(&mut self, path: Path, entity: Entity) {
        self.prune();
        self.log_state_trace();

        let id = Uuid::new_v4();
        let expiry = self.get_expiry();

        log::debug!("Saving entity with id {} at path {} with exipiry {}.", id, path, expiry);

        self.entities.insert(id, entity.clone());
        self.path_index.insert(&path.clone(), id);
        self.reverse_path_index.insert(id, path.clone());
        self.prune_index.entry(expiry).or_default().insert(id);
        self.reverse_prune_index.insert(id, expiry);
     }

     fn get(&mut self, path : &Path) -> Vec<Entity> {
        self.prune();
        self.refresh_entity_lifetime(path.to_string());
        self.log_state_trace();

        let ids = self.path_index.get_data(path, true).unwrap_or_default();
        ids.iter().filter_map(|id|  { self.entities.get(id).cloned() }).collect()
     }

     fn refresh_entity_lifetime(&mut self, path: Path) {
         let new_expiry = self.get_expiry();
         let ids = self.path_index.get_data(&path, self.refresh_child_entities).unwrap_or_default();
         for id in ids {
             self.prune_index.entry(new_expiry).or_default().insert(*id);
             if let Some(old_expiry) = self.reverse_prune_index.insert(id.clone(), new_expiry).as_ref() {
                 log::debug!("Refreshing entity with id {} at path {}. Former expiry {}, New expiry {}.", id, path, old_expiry, new_expiry);
                 if let Some(expiring_ids) = self.prune_index.get_mut(old_expiry) {
                     expiring_ids.remove(&id);
                 };
                 if self.prune_index.entry(*old_expiry).or_default().is_empty() {
                     self.prune_index.remove(old_expiry);
                 };
             };
         };
     }

     fn prune(&mut self) {
         let mut ids_to_expire : Vec<Id> = Vec::new();
         let mut past_expiry_timestamps : Vec<Timestamp> = Vec::new();
         for (expiry, ids) in self.prune_index.range(0..self.get_now()) {
             past_expiry_timestamps.push(*expiry);
             for id in ids {
                 ids_to_expire.push(id.clone());
             }
         };
         for expiry in &past_expiry_timestamps {
             self.prune_index.remove(&expiry);
         };
         for id in ids_to_expire {
             self.entities.remove(&id);
             let expiry = self.reverse_prune_index.remove(&id);
             if let Some(path) = self.reverse_path_index.remove(&id) {
                log::debug!("Pruning entity with id {} at path {}. Expired at {}.", id, path, expiry.unwrap_or(0));
                 let ids_at_path = self.path_index.remove(&path).unwrap_or(Vec::new());
                 let remaining_ids : Vec<&Uuid> = ids_at_path.iter().filter(|other_id| { **other_id != id }).collect();
                 for id in remaining_ids {
                     self.path_index.insert(&path, *id);
                 };
             };
         };
     }

     fn get_now(&self) -> Timestamp {
         match SystemTime::now().duration_since(UNIX_EPOCH) {
             Ok(n) => n.as_secs(),
             Err(_) => panic!("system time is before the start of the unix epoch!")
         }
     }

     fn get_expiry(&self) -> Timestamp {
         self.get_now() + self.lifetime
     }

     fn log_state_trace(&self) {
         log::trace!("**************** STATE **********************");
         log::trace!("Current time: {}", self.get_now());
         log::trace!("Lifetime: {}", self.lifetime);
         log::trace!("Refresh child entities: {}", self.refresh_child_entities);
         log::trace!("Entities {:#?}", self.entities);
         log::trace!("Reverse path index {:#?}", self.reverse_path_index);
         log::trace!("Prune index {:#?}", self.prune_index);
         log::trace!("Reverse prune index {:#?}", self.reverse_prune_index);
         log::trace!("*********************************************");
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

    pub(crate) fn create_entity(&mut self, key : Path, entity : Entity) {
        self.state.write().insert(key, entity);
    }

    pub(crate) fn get_entities(self, key : &Path) -> Vec<Entity> {
        self.state.write().get(key)
    }
}
