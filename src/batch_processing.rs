// src/batch_processing.rs
mod batch_processing;

pub use self::batch_processing::BatchProcessor;

pub mod batch_processing {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};
    use crate::transaction::Transaction;

    pub struct BatchProcessor {
        transaction_queue: Arc<Mutex<VecDeque<Transaction>>>,
        batch_size: usize,
    }

    impl BatchProcessor {
        pub fn new(batch_size: usize) -> Self {
            let transaction_queue = Arc::new(Mutex::new(VecDeque::new()));
            BatchProcessor {
                transaction_queue,
                batch_size,
            }
        }

        pub fn enqueue_transaction(&self, transaction: Transaction) {
            self.transaction_queue.lock().unwrap().push_back(transaction);
        }

        pub fn process_transactions(&self) -> Result<(), String> {
            let mut queue = self.transaction_queue.lock().unwrap();
            let mut batch = Vec::new();

            while let Some(transaction) = queue.pop_front() {
                batch.push(transaction);

                if batch.len() == self.batch_size {
                    // Process batch
                    self.submit_batch(batch)?;
                    batch.clear();
                }
            }

            if !batch.is_empty() {
                self.submit_batch(batch)?;
            }

            Ok(())
        }

        fn submit_batch(&self, batch: Vec<Transaction>) -> Result<(), String> {
            // Submit batch to blockchain
            Ok(())
        }
    }
}