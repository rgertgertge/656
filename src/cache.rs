// src/cache.rs
mod cache;

pub use self::cache::Cache;

pub mod cache {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    pub struct Cache {
        data_store: Arc<Mutex<HashMap<String, (String, Duration)>>>,
    }

    impl Cache {
        pub fn new() -> Self {
            let data_store = Arc::new(Mutex::new(HashMap::new()));
            Cache { data_store }
        }

        pub fn get_data(&self, key: &str) -> Option<String> {
            let mut data_store = self.data_store.lock().unwrap();
            if let Some((value, expiration)) = data_store.get(key) {
                if expiration > &Duration::from_secs(0) {
                    return Some(value.clone());
                } else {
                    data_store.remove(key);
                }
            }
            None
        }

        pub fn set_data(&self, key: String, value: String, ttl: Duration) {
            let mut data_store = self.data_store.lock().unwrap();
            data_store.insert(key, (value, ttl));
        }
    }
}