// src/transaction.rs
mod transaction;

pub use self::transaction::Transaction;

pub mod transaction {
    use std::sync::Arc;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Transaction {
        pub id: u64,
        pub from: String,
        pub to: String,
        pub amount: u64,
        pub signature: String,
    }

    impl Transaction {
        pub fn new(from: String, to: String, amount: u64, signature: String) -> Self {
            Transaction {
                id: 0, // 自动生成 ID
                from,
                to,
                amount,
                signature,
            }
        }

        pub async fn handle_transactions(&self) {
            // 处理交易逻辑
            println!("Handling transaction: {:?}", self);
        }

        pub fn validate_signature(&self) -> bool {
            // 验证交易签名
            true
        }

        pub fn update_state(&self) {
            // 更新状态
            println!("Updating state with transaction: {:?}", self);
        }
    }
}