use lru::LruCache;
use std::sync::{Arc, Mutex};

pub struct SearchCache {
    cache: Arc<Mutex<LruCache<String, Vec<u64>>>>,
}

impl SearchCache {
    pub fn new(capacity: usize) -> Self {
        SearchCache {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }

    pub fn get_or_insert(&self, key: String, search_fn: impl Fn() -> Vec<u64>) -> Vec<u64> {
        if let Some(cached) = self.cache.lock().unwrap().get(&key) {
            return cached.clone();
        }
        let results = search_fn();
        self.cache.lock().unwrap().put(key, results.clone());
        results
    }
}