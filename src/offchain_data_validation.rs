// src/offchain_data_validation.rs
mod offchain_data_validation;

pub use self::offchain_data_validation::OffchainDataValidation;

pub mod offchain_data_validation {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct OffchainDataValidation {
        data_store: Arc<Mutex<HashMap<String, String>>>,
    }

    impl OffchainDataValidation {
        pub fn new() -> Self {
            let data_store = Arc::new(Mutex::new(HashMap::new()));
            OffchainDataValidation { data_store }
        }

        pub fn store_data(&self, key: String, value: String) -> Result<(), String> {
            self.data_store.lock().unwrap().insert(key, value);
            Ok(())
        }

        pub fn validate_data(&self, key: &str, expected_value: &str) -> Result<bool, String> {
            let data_store = self.data_store.lock().unwrap();
            if let Some(value) = data_store.get(key) {
                Ok(value == expected_value)
            } else {
                Err("Data not found".to_string())
            }
        }
    }
}