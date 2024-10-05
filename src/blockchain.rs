// src/blockchain.rs
mod blockchain;

pub use self::blockchain::Blockchain;

pub mod blockchain {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use lru::LruCache;
    use serde::{Deserialize, Serialize};
    use crate::transaction::Transaction;
    use crate::solidity::SolidityContract;
    use rocksdb::{DB, Options};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Block {
        pub height: u64,
        pub prev_hash: String,
        pub transactions: Vec<Transaction>,
        pub hash: String,
    }

    pub struct Blockchain {
        db: DB,
        cache: Arc<Mutex<LruCache<u64, Block>>>,
        contracts: Arc<Mutex<HashMap<String, SolidityContract>>>,
    }

    impl Blockchain {
        pub fn new(db_path: &str) -> Result<Self, String> {
            let options = Options::default();
            let db = DB::open_default(db_path).map_err(|e| format!("Failed to open database: {}", e))?;

            let cache = Arc::new(Mutex::new(LruCache::new(100)));
            let contracts = Arc::new(Mutex::new(HashMap::new()));

            Ok(Blockchain {
                db,
                cache,
                contracts,
            })
        }

        pub fn initialize(&self) {
            let genesis_block = Block {
                height: 0,
                prev_hash: String::from("0"),
                transactions: Vec::new(),
                hash: String::from("genesis"),
            };

            let mut cache = self.cache.lock().unwrap();
            cache.put(0, genesis_block.clone());

            let mut contracts = self.contracts.lock().unwrap();
            contracts.insert(String::from("genesis"), SolidityContract::new("").unwrap());

            self.db.put(b"block_0", &serde_json::to_vec(&genesis_block).unwrap()).unwrap();
        }

        pub fn add_block(&self, block: Block) {
            let mut cache = self.cache.lock().unwrap();
            cache.put(block.height, block.clone());

            self.db.put(format!("block_{}", block.height), &serde_json::to_vec(&block).unwrap()).unwrap();
        }

        pub fn get_block(&self, height: u64) -> Option<Block> {
            let cache = self.cache.lock().unwrap();
            if let Some(block) = cache.get(&height) {
                return Some(block.clone());
            }

            let bytes = self.db.get(format!("block_{}", height)).unwrap()?;
            let block: Block = serde_json::from_slice(&bytes).unwrap();
            let mut cache = self.cache.lock().unwrap();
            cache.put(height, block.clone());
            Some(block)
        }

        pub fn validate_block(&self, block: &Block) -> bool {
            if block.height == 0 {
                return true; // 创世区块总是有效
            }

            if let Some(prev_block) = self.get_block(block.height - 1) {
                if block.prev_hash != prev_block.hash {
                    return false;
                }
            } else {
                return false;
            }

            // 验证交易
            for tx in &block.transactions {
                if !self.validate_transaction(tx) {
                    return false;
                }
            }

            true
        }

        pub fn validate_transaction(&self, tx: &Transaction) -> bool {
            // 验证交易签名等
            true
        }

        pub fn deploy_contract(&self, source_code: &str) -> Result<SolidityContract, String> {
            let contract = SolidityContract::new(source_code)?;
            let mut contracts = self.contracts.lock().unwrap();
            let contract_name = format!("contract_{}", contracts.len());
            contracts.insert(contract_name.clone(), contract.clone());
            Ok(contract)
        }

        pub fn get_contract(&self, name: &str) -> Option<&SolidityContract> {
            let contracts = self.contracts.lock().unwrap();
            contracts.get(name)
        }
    }
}