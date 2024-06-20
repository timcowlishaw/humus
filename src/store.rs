use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::{HashMap, BTreeMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

const LIFETIME : u64 = 60 * 1;

fn get_now() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("system time is before the start of the unix epoch!")
    }
}

fn get_expiry() -> u64 {
    get_now() + LIFETIME
}


pub(crate) type Entity = HashMap<String, String>;

pub(crate) type Entities = HashMap<String, Entity>;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Key {
    entity: String,
    attribute: String
}

type PruneIndex = BTreeMap<u64, HashSet<Key>>;
type ReversePruneIndex = HashMap<Key, u64>;

struct StoreState {
    entities: Entities,
    prune_index: PruneIndex,
    reverse_prune_index: ReversePruneIndex
}

impl StoreState {
     fn new() -> Self {
        StoreState {
            entities: HashMap::new(),
            prune_index: BTreeMap::new(),
            reverse_prune_index: HashMap::new()
        }
    }

     fn insert(&mut self, entity_key: String, entity: Entity) -> () {
        self.prune();
        self.entities.insert(entity_key.clone(), entity.clone());
        let expiry = get_expiry();
        let index_entry = self.prune_index.entry(expiry).or_default();
        for attribute_key in entity.keys() {
            let key = Key { entity: entity_key.clone(), attribute: attribute_key.to_string() };
            self.reverse_prune_index.insert(key.clone(), expiry);
            index_entry.insert(key);
        }
     }

     fn get(&mut self, key : &String) -> Option<Entity> {
        self.refresh_entity_lifetime(key);
        self.prune();
        self.entities.get(key).cloned()
     }

     fn get_all(&mut self) -> Entities {
         self.prune();
         self.entities.clone()
     }

     fn refresh_entity_lifetime(&mut self, entity_key : &String) -> () {
         let new_expiry = get_expiry();
         self.entities.get(entity_key).and_then(|entity| {
             for attribute_key in entity.keys() {
                 let key = Key { entity: entity_key.clone(), attribute: attribute_key.clone() };
                 self.reverse_prune_index.insert(key.clone(), new_expiry).as_ref().and_then(|old_expiry| {
                     self.prune_index.get_mut(old_expiry).and_then(|expiring_keys| {
                         expiring_keys.remove(&key);
                         Some(())
                     });
                     Some(())
                 });
             }
             Some(())
         });
     }

     fn prune(&mut self) -> () {
         let mut keys_to_expire : Vec<Key> = Vec::new();
         let mut past_expiries : Vec<u64> = Vec::new();
         for (expiry, keys) in self.prune_index.range(0..get_now()) {
             past_expiries.push(*expiry);
             for key in keys {
                 keys_to_expire.push(key.clone());
             }
         }
         for expiry in past_expiries {
             self.prune_index.remove(&expiry);
         }
         let mut entities_to_expire : Vec<String> = Vec::new();
         for key in keys_to_expire {
             self.reverse_prune_index.remove(&key);
             self.entities.get_mut(&key.entity).and_then(|entity| {
                 entity.remove(&key.attribute);
                 if entity.is_empty() {
                     entities_to_expire.push(key.entity);
                 }
                 Some(())
             });
         }
         for key in entities_to_expire {
             self.entities.remove(&key);
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
        self.state.write().insert(key, entity);
    }

    pub(crate) fn get_entity(self, key : &String) -> Option<Entity> {
        self.state.write().get(key)
    }

    pub(crate) fn get_entities(self) -> Entities {
        self.state.write().get_all()
    }
}
