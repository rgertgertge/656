// src/governance.rs
mod governance;

pub use self::governance::Governance;

pub mod governance {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use serde::{Deserialize, Serialize};
    use crate::blockchain::Blockchain;
    use crate::transaction::Transaction;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Proposal {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub votes: HashMap<String, bool>,
    }

    pub struct Governance {
        proposals: Arc<Mutex<HashMap<u64, Proposal>>>,
        blockchain: Arc<Blockchain>,
    }

    impl Governance {
        pub fn new(blockchain: Arc<Blockchain>) -> Self {
            Governance {
                proposals: Arc::new(Mutex::new(HashMap::new())),
                blockchain: blockchain,
            }
        }

        pub fn create_proposal(&self, title: String, description: String) -> Result<u64, String> {
            let proposal_id = self.proposals.lock().unwrap().len() as u64 + 1;
            let proposal = Proposal {
                id: proposal_id,
                title,
                description,
                votes: HashMap::new(),
            };

            self.proposals.lock().unwrap().insert(proposal_id, proposal.clone());

            Ok(proposal_id)
        }

        pub fn vote_on_proposal(&self, proposal_id: u64, voter: String, vote: bool) -> Result<bool, String> {
            let mut proposals = self.proposals.lock().unwrap();
            if let Some(proposal) = proposals.get_mut(&proposal_id) {
                proposal.votes.insert(voter, vote);
                Ok(true)
            } else {
                Err("Proposal not found".to_string())
            }
        }

        pub fn get_proposal(&self, proposal_id: u64) -> Option<Proposal> {
            let proposals = self.proposals.lock().unwrap();
            proposals.get(&proposal_id).cloned()
        }

        pub fn list_proposals(&self) -> Vec<Proposal> {
            let proposals = self.proposals.lock().unwrap();
            proposals.values().cloned().collect()
        }

        pub fn execute_proposal(&self, proposal_id: u64) -> Result<bool, String> {
            let proposal = self.get_proposal(proposal_id).ok_or("Proposal not found")?;
            let total_votes = proposal.votes.len();
            let yes_votes = proposal.votes.iter().filter(|(_, &vote)| vote).count();

            if yes_votes > total_votes / 2 {
                // 执行提案
                Ok(true)
            } else {
                Err("Proposal did not pass".to_string())
            }
        }
    }
}