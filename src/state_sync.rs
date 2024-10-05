// src/state_sync.rs
mod state_sync;

pub use self::state_sync::StateSync;

pub mod state_sync {
    use std::sync::Arc;
    use crate::blockchain::Blockchain;

    pub struct StateSync {
        state_root: Arc<String>,
        blockchain: Arc<Blockchain>,
    }

    impl StateSync {
        pub fn new(blockchain: Arc<Blockchain>) -> Self {
            StateSync {
                state_root: Arc::new(String::from("")),
                blockchain,
            }
        }

        pub async fn sync_state(&self) {
            // 状态同步逻辑
            println!("Syncing state...");
            self.blockchain.initialize();
        }

        pub fn update_state_root(&mut self, new_root: String) {
            self.state_root = Arc::new(new_root);
        }
    }
}