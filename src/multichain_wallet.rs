// src/multichain_wallet.rs
mod multichain_wallet;

pub use self::multichain_wallet::MultiChainWallet;

pub mod multichain_wallet {
    use ethers::prelude::*;
    use substrate_api_client::Api;
    use solana_sdk::signature::Keypair;
    use std::sync::Arc;

    pub struct MultiChainWallet {
        ethereum_wallet: LocalWallet,
        substrate_wallet: Api,
        solana_wallet: Keypair,
    }

    impl MultiChainWallet {
        pub fn new(ethereum_private_key: String, substrate_endpoint: &str) -> Self {
            let ethereum_wallet = LocalWallet::from_str(&ethereum_private_key).expect("Failed to create Ethereum wallet");
            let substrate_wallet = Api::new(substrate_endpoint).expect("Failed to create Substrate API");
            let solana_wallet = Keypair::new();

            MultiChainWallet {
                ethereum_wallet,
                substrate_wallet,
                solana_wallet,
            }
        }

        pub fn sign_ethereum_transaction(&self, transaction: TransactionRequest) -> Result<SignedTransaction, String> {
            let signed_transaction = self.ethereum_wallet.sign(transaction).map_err(|e| format!("Failed to sign Ethereum transaction: {}", e))?;
            Ok(signed_transaction)
        }

        pub fn sign_substrate_transaction(&self, transaction: substrate_api_client::types::Extrinsic) -> Result<substrate_api_client::types::Extrinsic, String> {
            let signed_transaction = self.substrate_wallet.sign_extrinsic(transaction).map_err(|e| format!("Failed to sign Substrate transaction: {}", e))?;
            Ok(signed_transaction)
        }

        pub fn sign_solana_transaction(&self, transaction: solana_sdk::transaction::Transaction) -> Result<solanaj::transaction::SignedTransaction, String> {
            let signed_transaction = solanaj::transaction::sign_transaction(transaction, vec![&self.solana_wallet]).map_err(|e| format!("Failed to sign Solana transaction: {}", e))?;
            Ok(signed_transaction)
        }
    }
}