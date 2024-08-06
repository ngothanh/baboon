use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

use count_min_sketch::CountMinSketch8;

trait ConcurrentCache<K, V> {
    fn insert(&self, key: K, value: V);

    fn get(&self, key: &K) -> Option<Arc<V>>;
}

pub struct LFUCache<K, V>
where
    K: Eq + Hash,
{
    inner: RwLock<Inner<K, V>>,
}

pub struct Inner<K, V>
where
    K: Eq + Hash,
{
    main_storage: HashMap<K, Arc<V>>,
    frequency_sketch: CountMinSketch8<K>,
}

impl<K, V> Inner<K, V>
where
    K: Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        let cache = HashMap::with_capacity(capacity);
        let sketch =
            CountMinSketch8::new(capacity, 0.95, 10.0)
                .expect("Failed to create frequency sketch");

        Self {
            main_storage: cache,
            frequency_sketch: sketch,
        }
    }
    fn insert(&mut self, key: K, value: V) {
        self.frequency_sketch.estimate(&key);
        self.main_storage.insert(key, Arc::new(value));
    }

    fn get(&mut self, key: &K) -> Option<Arc<V>> {
        self.frequency_sketch.increment(key);
        self.main_storage.get(key)
            .map(|v| Arc::clone(v))
    }
}

impl<K, V> LFUCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(Inner::new(capacity))
        }
    }

    pub fn inner_mut(&self) -> RwLockWriteGuard<'_, Inner<K, V>> {
        self.inner.write()
            .expect("Unable to acquire write lock")
    }
}

impl<K, V> ConcurrentCache<K, V> for LFUCache<K, V>
where
    K: Eq + Hash,
{
    fn insert(&self, key: K, value: V) {
        self.inner_mut().insert(key, value);
    }

    fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner_mut().get(key)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{ConcurrentCache, LFUCache};

    #[test]
    fn given_newly_insert_kv_when_getting_value_back_then_return_correctly() {
        //given
        let key = "test";
        let value = 10;
        let cache = LFUCache::new(10);
        cache.insert(key, value);

        //when
        let expect = cache.get(&key).unwrap();

        //then
        assert_eq!(expect, Arc::new(value))
    }
}