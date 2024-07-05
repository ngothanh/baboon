use std::sync::Arc;

trait ConcurrentCache<K, V> {
    fn insert(&self, key: K, value: V);

    fn get(&self, key: &K) -> Arc<&V>;
}