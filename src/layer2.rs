// src/layer2.rs
mod layer2;

pub use self::layer2::Layer2;

pub mod layer2 {
    use optimal_rollup::OptimisticRollup;
    use zksync::ZKSync;
    use std::sync::Arc;

    pub struct Layer2 {
        optimistic_rollup: Arc<OptimisticRollup>,
        zksync: Arc<ZKSync>,
    }

    impl Layer2 {
        pub fn new(optimistic_rollup: Arc<OptimisticRollup>, zksync: Arc<ZKSync>) -> Self {
            Layer2 {
                optimistic_rollup,
                zksync,
            }
        }

        pub fn submit_transaction_optimistic_rollup(&self, transaction: String) -> Result<(), String> {
            self.optimistic_rollup.submit_transaction(transaction)?;
            Ok(())
        }

        pub fn submit_transaction_zksync(&self, transaction: String) -> Result<(), String> {
            self.zksync.submit_transaction(transaction)?;
            Ok(())
        }
    }
}