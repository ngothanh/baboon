use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

trait ConcurrentCache<K, V> {
    fn insert(&self, key: K, value: V);

    fn get(&self, key: &K) -> Option<Arc<&V>>;
}

pub struct LFUCache<K, V>
where
    K: Eq + Hash,
{
    main_storage: RwLock<HashMap<K, Arc<V>>>,
}

impl<K, V> LFUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            main_storage: RwLock::new(HashMap::with_capacity(capacity))
        }
    }
}

impl<K, V> ConcurrentCache<K, V> for LFUCache<K, V>
where
    K: Eq + Hash,
{
    fn insert(&self, key: K, value: V) {
        let mut m = self.main_storage
            .write()
            .expect("Cannot acquire write lock on the map");
        m.insert(key, Arc::new(value));
    }

    fn get(&self, key: &K) -> Option<Arc<&V>> {
        let m = self.main_storage
            .read()
            .expect("Cannot acquire read lock on the map");

        m.get(key)
            .map(|v| Arc::clone(v))
    }
}